use std::path::Path as Path;
use std::time::Instant;
use std::collections::HashSet;
use std::collections::VecDeque;
use crate::common::io;

fn set_values_of_memory_and_sum(instructions: &Vec<String>) -> i128 {
    let size = (2 as i64).pow(36);
    let mut memory: Vec<i64> = vec![0;  size as usize];
    let mut mask_1: i64 = 0;
    let mut mask_0: i64 = 0;
    let mut set: HashSet<usize> = HashSet::new();
    let mut sum: i128 = 0;

    for instruction in instructions {
        log::debug!("Processing instruction {}", instruction);
        let mut modified_instruction = instruction.replace("mask = ", "mask;");
        modified_instruction = modified_instruction.replace("mem[", "mem;");
        modified_instruction = modified_instruction.replace("] = ", ";");

        let tokens: Vec<&str> = modified_instruction.split(';').collect();
        let command = tokens[0];

        match command {
            "mem" => {
                let index: usize = tokens[1].parse().unwrap();
                let value: i64 = tokens[2].parse().unwrap();

                if set.contains(&index) {
                    sum -= memory[index] as i128;
                }
                set.insert(index);
                memory[index] = (value & mask_0) | mask_1;
                sum += memory[index] as i128;
                log::debug!("Setting memory location {} with {}, original value {}", index, memory[index], value);
            },
            "mask" => {
                let bin_mask_1 = tokens[1].replace("X", "0");
                let bin_mask_0 = tokens[1].replace("X", "1");
                mask_1 = i64::from_str_radix(&bin_mask_1, 2).unwrap();
                mask_0 = i64::from_str_radix(&bin_mask_0, 2).unwrap();
                log::debug!("Setting mask to {}, mask_0 {} mask_1 {}", tokens[1], mask_0, mask_1);
            },
            _ => {
                panic!("Instruction not known, something is messed up");
            }
        }
    }

    return sum;
}

fn generate_addresses(mask: &String, address: &String) -> Vec<usize> {
    log::trace!("Mask {} adddress {}", mask, address);
    let mut alternatives: VecDeque<(Vec<char>, usize)> = VecDeque::new();
    let mut addresses: Vec<usize> = Vec::new();
    let mut address_chars: Vec<char> = vec!['0'; mask.len() - address.len()];
    address_chars.append(&mut address.chars().collect());
    let mask_chars: Vec<char> = mask.chars().collect();
    
    alternatives.push_back((mask_chars.clone(), 0));

    while ! alternatives.is_empty() {
        let entry = alternatives.pop_front().unwrap();
        log::trace!("Entry {:?} index {}", entry.0, entry.1);
        if entry.0.len() == entry.1 {
            let complete_address_string = entry.0.into_iter().collect::<String>();
            let complete_address = usize::from_str_radix(&complete_address_string, 2).unwrap();
            log::trace!("Adding adddress {}", complete_address);
            addresses.push(complete_address);
        } else {
            if entry.0[entry.1] == 'X' {
                let mut clone_1 = entry.0.clone();
                clone_1[entry.1] = '1';
                let mut clone_0 = entry.0.clone();
                clone_0[entry.1] = '0';
                alternatives.push_back((clone_1, entry.1 + 1));
                alternatives.push_back((clone_0, entry.1 + 1));
            } else {
                let mut clone = entry.0.clone();
                if entry.1 < address_chars.len() && address_chars[entry.1] == '1' {
                    clone[entry.1] = '1';
                } 
                alternatives.push_back((clone, entry.1 + 1));
            }
        }
    }
    log::debug!("Generated {} mirror addresses", addresses.len());
    return addresses;
}

fn set_values_of_memory_and_sumv2(instructions: &Vec<String>) -> i128 {
    let size = (2 as i64).pow(36);
    let mut memory: Vec<i64> = vec![0;  size as usize];
    let mut mask: String = "".to_string();
    let mut set: HashSet<usize> = HashSet::new();
    let mut sum: i128 = 0;

    for instruction in instructions {
        log::debug!("Processing instruction {}", instruction);
        let mut modified_instruction = instruction.replace("mask = ", "mask;");
        modified_instruction = modified_instruction.replace("mem[", "mem;");
        modified_instruction = modified_instruction.replace("] = ", ";");

        let tokens: Vec<&str> = modified_instruction.split(';').collect();
        let command = tokens[0];

        match command {
            "mem" => {
                let address: String = format!("{:b}", tokens[1].parse::<usize>().unwrap());
                let value: i64 = tokens[2].parse().unwrap();

                for memory_address in generate_addresses(&mask, &address) {
                    log::debug!("Setting memory location {} with {}", memory_address, value);
                    if set.contains(&memory_address) {
                        sum -= memory[memory_address] as i128;
                    }
                    set.insert(memory_address);
                    memory[memory_address] = value;
                    sum += memory[memory_address] as i128;
                }
            },
            "mask" => {
                mask = tokens[1].to_string();
                log::debug!("Setting mask to {}", mask);
            },
            _ => {
                panic!("Instruction not known, something is messed up");
            }
        }
    }

    return sum;
}

fn part1(lines: &Vec<String>) -> i128 {
    log::info!("Running Part 1");
    let sum = set_values_of_memory_and_sum(&lines);
    log::info!("Found the sum to be {}", sum);
    return sum;
}

fn part2(lines: &Vec<String>) -> i128 {
    log::info!("Running Part 2");
    let sum = set_values_of_memory_and_sumv2(&lines);
    log::info!("Found the sum to be {}", sum);
    return sum;
}

pub fn run(filename: impl AsRef<Path>) {
    let lines = io::lines_from_file(filename);
    let now = Instant::now();
    part1(&lines);
    log::info!("Part 1: {}ms", now.elapsed().as_millis());
    part2(&lines);
    log::info!("Part 1 + 2: {}ms", now.elapsed().as_millis());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let instructions: Vec<String> = vec![
            "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X",
            "mem[8] = 11",
            "mem[7] = 101",
            "mem[8] = 0"
        ].iter().map(|x| x.to_string()).collect();

        assert_eq!(165, part1(&instructions));
    }

    #[test]
    fn part2_test() {
        let instructions: Vec<String> = vec![
            "mask = 000000000000000000000000000000X1001X",
            "mem[42] = 100",
            "mask = 00000000000000000000000000000000X0XX",
            "mem[26] = 1"
        ].iter().map(|x| x.to_string()).collect();

        assert_eq!(208, part2(&instructions));
    }
}