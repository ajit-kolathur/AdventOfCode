use std::path::Path as Path;
use std::collections::HashMap;

use crate::common::convertor;
use crate::common::io;

fn part1(adapters: &Vec<i16>) -> i32 {
    log::info!("Running Part 1");

    let mut countDict: HashMap<i16, i16> = HashMap::new();
    let mut ordered = adapters.clone();
    ordered.sort();

    log::info!("Found that the ordered list of adapters is {:?} of length {}", ordered, ordered.len());

    for i in 0..ordered.len()-1 {
        let diff = ordered[i+1] - ordered[i];
        if !countDict.contains_key(&(diff)) {
            countDict.insert(diff, 0);
        }
        *countDict.get_mut(&diff).unwrap() += 1;
    }

    log::info!("Found one diff cases to be {} and three diff to be {}", countDict[&1] + 1, countDict[&3]+1);
    return (countDict[&1] as i32 + 1) * (countDict[&3] as i32 + 1);
}

fn part2(adapters: &Vec<i16>) -> i64 {
    log::info!("Running Part 2");
    let mut sorted_adapters = adapters.clone();
    sorted_adapters.sort();

    let mut table: HashMap<i16, i64> = HashMap::new();
    
    // ways to get to Max + 3 is always 1
    let start: i16 = 0;
    let max: i16 = *sorted_adapters.last().unwrap();
    let end: i16 = max + 3;

    table.insert(start, 1);

    for adapter in sorted_adapters {
        let prev_1 = adapter - 1;
        let prev_2 = adapter - 2;
        let prev_3 = adapter - 3;

        let mut count = 0;
        if table.contains_key(&prev_1) {
            count += table[&prev_1];
        }
        if table.contains_key(&prev_2) {
            count += table[&prev_2];
        }
        if table.contains_key(&prev_3) {
            count += table[&prev_3];
        }

        table.insert(adapter, count);
    }

    log::info!("Found a total of {}", table[&max]);
    return table[&max];
}

pub fn run(filename: impl AsRef<Path>) {
    let adapters = convertor::vector_str_to_int(io::lines_from_file(filename));
    part1(&adapters);
    part2(&adapters);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let mut adapters: Vec<i16> = vec![28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35, 8, 17, 7, 9, 4, 2, 34, 10, 3];
        adapters.sort();
        assert_eq!(220, part1(&adapters));
    }

    #[test]
    fn part2_test() {
        env_logger::init();
        let mut adapters: Vec<i16> = vec![28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35, 8, 17, 7, 9, 4, 2, 34, 10, 3];
        adapters.sort();
        assert_eq!(19208, part2(&adapters));
    }
}