use std::path::Path as Path;
use std::time::Instant;
use std::collections::HashMap;
use crate::common::io;

fn predict_nth_number(start_list: &Vec<i64>, nth_number: i64) -> i64 {
    let mut memory: HashMap<i64, i64> = HashMap::new();
    let mut counter = 0;
    let mut last_spoken = 0;
    
    // Start list
    for number in start_list {
        counter += 1;
        memory.insert(*number, counter);
        last_spoken = *number;
        log::debug!("Counter {} Last Spoken {}", counter, last_spoken);
    }

    // Play it out until counter is the nth number
    while counter < nth_number {
        let new_last_spoken: i64;
        if memory.contains_key(&last_spoken) {
            new_last_spoken = counter - memory[&last_spoken];
        } else {
            new_last_spoken = 0;
        }
        memory.insert(last_spoken, counter);
        last_spoken = new_last_spoken;
        counter += 1;
        log::debug!("Counter {} Last Spoken {}", counter, new_last_spoken);
    }
    
    return last_spoken;
}

fn part1(start_list: &Vec<i64>) -> i64 {
    log::info!("Running Part 1");
    let last_spoken = predict_nth_number(&start_list, 2020);
    log::info!("Found the last spoken number to be {}", last_spoken);
    return last_spoken;
}

fn part2(start_list: &Vec<i64>) -> i64 {
    log::info!("Running Part 2");
    let last_spoken = predict_nth_number(&start_list, 30000000);
    log::info!("Found the last spoken number to be {}", last_spoken);
    return last_spoken;
}

pub fn run(filename: impl AsRef<Path>) {
    let numbers: Vec<i64> = 
    io::lines_from_file(filename)
    .first()
    .unwrap()
    .split(',')
    .map(|x| x.parse().unwrap()).collect();
    let now = Instant::now();
    part1(&numbers);
    log::info!("Part 1: {}ms", now.elapsed().as_millis());
    part2(&numbers);
    log::info!("Part 1 + 2: {}ms", now.elapsed().as_millis());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        env_logger::init();
        assert_eq!(0, part1(&vec![0,3,6]));
        assert_eq!(436, part1(&vec![0,3,6]));
    }
}