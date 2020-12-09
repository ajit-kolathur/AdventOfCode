use std::path::Path as Path;
use std::iter::FromIterator;

use crate::common::convertor;
use crate::common::search;
use crate::common::io;

fn is_valid_next_number(numbers: &Vec<i64>, next: i64) -> bool {
    let mut sorted: Vec<i64> = numbers.clone();
    sorted.sort();

    for number in numbers {
        let find = next - number;
        let index = search::binary_search(&sorted, find);

        if index >= 0 {
            return true;
        }
    }

    return false;
}

fn part1(numbers: &Vec<i64>, preamble: i64) -> i32 {
    log::info!("Running Part 1");
    for i in 0..numbers.len() - (preamble as usize) - 1 { 
        let start = i;
        let end = i + preamble as usize;
        let next = numbers[end+1];
        log::debug!("Running on {} to {} next number is {}", start, end, next);

        let part: Vec<i64> = Vec::from_iter(numbers[start..end+1].iter().cloned());

        if !is_valid_next_number(&part, next) {
            log::info!("Found number breaks the rule {}", next);
            return (end+1) as i32;
        }
    }
    
    return -1;
}

fn part2(numbers: &Vec<i64>, failure: i32) -> i64 {
    log::info!("Running Part 2");
    let length = failure as usize;
    log::info!("Length is {}", length);
    let mut cumulative: Vec<i64> = vec![0; length];

    let mut sum: i64 = 0;
    for i in 0..length {
        sum += numbers[i as usize];
        log::debug!("i: {} sum: {}", i, sum);
        cumulative[i as usize] = sum;
    }

    let sum_to_find = numbers[(failure) as usize];
    log::debug!("Looking for {}", sum_to_find);

    for i in 0..cumulative.len() {
        let find = sum_to_find + cumulative[i];
        
        if find < 0 {
            continue;
        }

        log::debug!("Trying to find {} and {}", find, cumulative[i]);

        let j = search::binary_search(&cumulative, find);
        if j!= -1 && i != j as usize {            
            let mut min = 10000000000;
            let mut max = -1;

            for index in i+1..(j+1) as usize {
                if min > numbers[index] {
                    min = numbers[index];
                }

                if max < numbers[index] {
                    max = numbers[index];
                }
            }
            log::info!("Encryption weakness in your XMAS-encrypted list of numbers {}", min + max);
            return min + max;
        }
    }

    return -1;
}

pub fn run(filename: impl AsRef<Path>) {
    let numbers = convertor::vector_str_to_int64(io::lines_from_file(filename));
    part1(&numbers, 25);
    part2(&numbers, part1(&numbers, 25));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_valid_next_number_test() {
        let numbers = vec![35, 20, 15, 25, 47];
        let not = vec![95, 102, 117, 150, 182];
        assert_eq!(true, is_valid_next_number(&numbers, 40));
        assert_eq!(false, is_valid_next_number(&not, 40));
    }

    #[test]
    fn part1_test() {
        let numbers = vec![35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309, 576];
        assert_eq!(14, part1(&numbers, 5));
    }

    #[test]
    fn part2_test() {
        env_logger::init();
        let numbers = vec![35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309, 576];
        let fail = part1(&numbers, 5);
        log::debug!("Failure was at {}, Failing number was {}", fail, numbers[fail as usize]);
        assert_eq!(62, part2(&numbers, fail));
    }
}