use std::path::Path as Path;
use std::iter::FromIterator;
use std::collections::HashSet;

use crate::common::io;
use crate::common::convertor;

fn part1(numbers: &Vec<i16>) -> f32 {
    log::info!("Running Part 1");
    let lookup: HashSet<i16> = HashSet::from_iter(numbers.iter().cloned());

    for number in numbers.iter() {
        let find = 2020 - number;
        log::debug!("Processing {}, looking for {}", number, find);

        if lookup.contains(&find) {
            log::info!("Found {}", find);
            let result: f32 = *number as f32 * find as f32;
            log::info!("Multiplication result is {}", result);
            return result;
        }
    }
    log::error!("Didnt find a pair such that the sum is 2020");
    return -1.0;
}

fn part2(numbers: &Vec<i16>) -> f32 {
    log::info!("Running Part 2");
    let lookup: HashSet<i16> = HashSet::from_iter(numbers.iter().cloned());
    let n = numbers.len();

    for i in 0..n {
        for j in i+1..n {
            
            if numbers[i] + numbers[j] >= 2020 {
                continue;
            }

            let find = 2020 - numbers[i] - numbers[j];
            log::debug!("Processing {} {}, looking for {}", numbers[i], numbers[j], find);

            if lookup.contains(&find) {
                log::info!("Found a trio {}, {} and {}", numbers[i], numbers[j], find);
                let result = (numbers[i] as f32) * (numbers[j] as f32) * (find as f32);
                log::info!("Multiplication result is {}", result);
                return result;
            }
        }
    }
    log::error!("Didnt find a pair such that the sum is 2020");
    return -1.0;
}

pub fn run(filename: impl AsRef<Path>) {
    // Sorted number list probably makes it easier find a pair
    let mut numbers: Vec<i16> = convertor::vector_str_to_int(io::lines_from_file(filename));
    numbers.sort();

    // Pick a numeber and binary search for corresponding pair for 2020 - number
    part1(&numbers);

    // Pick a pair of numbers and try to find a third such that they sum up to 2020
    part2(&numbers);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let mut numbers = vec![1721, 979, 366, 299, 675, 1456];
        numbers.sort();
        assert_eq!(514579 as f32, part1(&numbers));
    }

    #[test]
    fn part2_test() {
        let mut numbers = vec![1721, 979, 366, 299, 675, 1456];
        numbers.sort();
        assert_eq!(241861950 as f32, part2(&numbers));
    }
}