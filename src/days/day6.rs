use std::path::Path as Path;
use std::time::Instant;
use std::collections::HashSet;

use crate::common::io;
use crate::common::convertor;

// Chunk replies into group, per group get individual replies, union and sum union sizes
fn part1(lines: &Vec<String>) -> i32 {
    log::info!("Running Part 1");
    let mut count = 0;

    for line in lines {
        let mut union_questions: HashSet<char> = HashSet::new();
        let individual_answers: Vec<&str> = line.split(' ').collect();
        
        for individual_answer in individual_answers {
            let individual_answer_set: HashSet<char> = individual_answer.chars().collect();
            union_questions.extend(&individual_answer_set);
        }
        count += union_questions.len();
    }

    log::info!("The sum of these counts is {}", count);
    return count as i32;
}

// Chunk replies into group, per group find number of common respones
fn part2(lines: &Vec<String>) -> i32 {
    log::info!("Running Part 2");
    let mut count = 0;

    for line in lines {
        let individual_answers: Vec<&str> = line.split(' ').collect();
        let mut group_question_array: [i32; 26] = [0; 26];
        let token_count = individual_answers.len();

        for individual_answer in individual_answers {
            for character in individual_answer.chars() {
                group_question_array[(character as u32 - 'a' as u32) as usize] += 1;
            }
        }

        for i in 0..26 {
            if group_question_array[i] == token_count as i32 {
                count += 1;
            }
        }
    }

    log::info!("The sum of these counts is {}", count);
    return count as i32;
}

pub fn run(filename: impl AsRef<Path>) {
    let lines = io::lines_from_file(filename);
    let chunks = convertor::chunk_parts(&lines);
    let now = Instant::now();
    part1(&chunks);
    log::info!("Part 1: {}ms", now.elapsed().as_millis());
    part2(&chunks);
    log::info!("Part 1 + 2: {}ms", now.elapsed().as_millis());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let lines = vec![
            "abc",
            "",
            "a",
            "b",
            "c",
            "",
            "ab",
            "ac",
            "",
            "a",
            "a",
            "a",
            "a",
            "",
            "b"
        ].iter().map(|x| x.to_string()).collect();
        let chunks = convertor::chunk_parts(&lines);
        assert_eq!(11, part1(&chunks));
    }

    #[test]
    fn part2_test() {
        let lines = vec![
            "abc",
            "",
            "a",
            "b",
            "c",
            "",
            "ab",
            "ac",
            "",
            "a",
            "a",
            "a",
            "a",
            "",
            "b"
        ].iter().map(|x| x.to_string()).collect();
        let chunks = convertor::chunk_parts(&lines);
        assert_eq!(6, part2(&chunks));
    }
}