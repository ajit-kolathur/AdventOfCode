use std::collections::HashSet;
use std::path::Path as Path;

use crate::common::io;

fn run_program_until_loop(instructions: &Vec<String>) -> (i32, i32) {
    let mut visited: HashSet<i32> = HashSet::new();

    let mut ip = 0;
    let mut acc = 0;

    while !visited.contains(&ip) && ip < instructions.len() as i32{
        let tokens: Vec<&str> = instructions[ip as usize].split(' ').collect();
        let instruction = tokens[0].trim();
        visited.insert(ip);

        match instruction {
            "acc" => { 
                let operrand: i32 = tokens[1].trim().parse().unwrap();
                acc += operrand;
                ip += 1;
                continue;
            },
            "jmp" => { 
                let operrand: i32 = tokens[1].trim().parse().unwrap();
                ip += operrand;
                continue;
            },
            "nop" => { 
                ip += 1;
                continue;
            },
            _     => {
                panic!("crash and burn");
            }
        }
    }
    return (acc, ip);
}

// Treat it like a simple turing machine or cpu
// instructions result in a accumulator increase (single global register)
// instructions are executed serially unless jump encountered
// if a line is executed twice stop and return ip and accumulator there
fn part1(instructions: &Vec<String>) -> i32 {
    log::info!("Running Part 1");
    let result = run_program_until_loop(&instructions);
    let acc = result.0;
    log::info!("Found accumulater value to be {} before loop", acc);
    return acc;
}

// Given a buggy program, we need to find line to replace jmp with nop or opposite
// the outcome should be that the program executes to completion, try replacing these
// one by one until you find one that results in program terminattion
fn part2(instructions: &Vec<String>) -> i32 {
    log::info!("Running Part 2");
    
    for i in 0..instructions.len() {
        let mut clone = instructions.to_vec();
        if instructions[i].contains("jmp") {
            clone[i] = clone[i].replace("jmp", "nop");
        } else if instructions[i].contains("nop") {
            clone[i] = clone[i].replace("nop", "jmp");
        }
        let result = run_program_until_loop(&clone);
        if  result.1 as usize == instructions.len() {
            log::info!("Found buggy line to be {} before loop", i + 1);
            log::info!("Instruction: {}", instructions[i]);
            log::info!("Accumulator: {}", result.0);
            return (i + 1) as i32;
        }
    }

    return -1;
}

pub fn run(filename: impl AsRef<Path>) {
    let lines = io::lines_from_file(filename);
    part1(&lines);
    part2(&lines);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let instructions: Vec<String> = vec![
            "nop +0",
            "acc +1",
            "jmp +4",
            "acc +3",
            "jmp -3",
            "acc -99",
            "acc +1",
            "jmp -4",
            "acc +6",
        ].iter().map(|x| x.to_string()).collect();
        assert_eq!(5, part1(&instructions));
    }

    #[test]
    fn part2_test() {
        let instructions: Vec<String> = vec![
            "nop +0",
            "acc +1",
            "jmp +4",
            "acc +3",
            "jmp -3",
            "acc -99",
            "acc +1",
            "jmp -4",
            "acc +6",
        ].iter().map(|x| x.to_string()).collect();
        assert_eq!(8, part2(&instructions));
    }
}