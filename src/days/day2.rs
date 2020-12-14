use std::time::Instant;
use std::path::Path as Path;

use crate::common::io;

struct PasswordRule {
    min: i8,
    max: i8,
    character: char
}

type PasswordEntry = (String, PasswordRule);

// Parse a line to extract PasswordRule or fail
fn parse_rule_string(rule_string: &str) -> Result<PasswordRule, &'static str> {
    let tokens: Vec<&str> = rule_string.split_whitespace().collect();
    let range: Vec<&str> = tokens[0].trim().split('-').collect();
    let min: i8 = range[0].trim().parse().unwrap();
    let max: i8 = range[1].trim().parse().unwrap();
    let character: char = tokens[1].trim().chars().next().unwrap();
    log::debug!("input was {}, rule generated min {}, max {}, char {}", rule_string, min, max, character);
    return Ok(PasswordRule {min: min, max: max, character: character});
}

// Generate PasswordEntry for each line
fn parse_file(lines: &Vec<String>) -> Vec<PasswordEntry> {
    let mut entries: Vec<PasswordEntry> = Vec::new();
    
    for line in lines {
        let parts: Vec<&str> = line.split(':').collect();
        let rule_str = parts[0].trim();
        let password = parts[1].trim().to_string();
        log::debug!("input was {}, Found password {} and rule_str {}", line, password, rule_str);

        let rule = parse_rule_string(rule_str);
        if rule.is_ok() {
            entries.push((password, rule.unwrap()))
        }
    }

    return entries;
}

// Check that the count of given character is greater than equal to min value
// Check that the count of given chatacter is less than equal to max value
fn check_type1_validity(entry: &PasswordEntry) -> bool{
    let occurences: Vec<char> = entry.0.chars().filter(|x| *x == entry.1.character).collect();
    let count = occurences.len() as i8;
    log::debug!(
        "Checking rule for {}, looking for character {} found count {}, expected min {}, max {}",
        entry.0,
        entry.1.character,
        count,
        entry.1.min,
        entry.1.max);
    return count >= entry.1.min && count <= entry.1.max;
}

// Check that the character at nth index obeys a rule, and only 1 of two rules are valid
fn check_type2_validity(entry: &PasswordEntry) -> bool{
    let mut count = 0;
    if entry.0.chars().nth((entry.1.min - 1) as usize).unwrap() == entry.1.character {
        count+= 1;
    }
    if entry.0.chars().nth((entry.1.max - 1) as usize).unwrap() == entry.1.character {
        count+= 1;
    }

    log::debug!(
        "Checking rule for {}, looking for character {} at {} or {}, count of finds {}",
        entry.0,
        entry.1.character,
        entry.1.min,
        entry.1.max,
        count);
    return count == 1;
}

fn part1(lines: &Vec<String>) -> i32 {
    log::info!("Running Part 1");
    let entries = parse_file(&lines);
    let valid_entries: Vec<&PasswordEntry> = entries.iter().filter(|x| check_type1_validity(x)).collect();
    let count = valid_entries.len() as i32;

    log::info!("Found {} valid passwords in input file", count);
    return count;
}

fn part2(lines: &Vec<String>) -> i32 {
    log::info!("Running Part 2");
    let entries = parse_file(&lines);
    let valid_entries: Vec<&PasswordEntry> = entries.iter().filter(|x| check_type2_validity(x)).collect();
    let count = valid_entries.len() as i32;

    log::info!("Found {} valid passwords in input file", count);
    return count;
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
    fn parse_rule_string_test() {
        let rule_str = "1-3 a: abcde";
        let password_rule = parse_rule_string(&rule_str);
        assert_eq!(true, password_rule.is_ok());

        let unwrapped = password_rule.unwrap();
        assert_eq!(1, unwrapped.min);
        assert_eq!(3, unwrapped.max);
        assert_eq!('a', unwrapped.character);
    }

    #[test]
    fn parse_file_test() {
        let lines = vec!["1-3 a: abcde"].iter().map(|x| x.to_string()).collect();
        let password_entries = parse_file(&lines);

        let entry = password_entries.get(0).unwrap();
        assert_eq!("abcde".to_string(), entry.0);
        assert_eq!(1, entry.1.min);
        assert_eq!(3, entry.1.max);
        assert_eq!('a', entry.1.character);
    }

    #[test]
    fn check_type1_validity_test() {
        let lines = vec!["1-3 a: abcde", "1-3 b: cdefg"].iter().map(|x| x.to_string()).collect();
        let password_entries = parse_file(&lines);

        let first = password_entries.get(0).unwrap();
        assert_eq!(true, check_type1_validity(&first));

        let second = password_entries.get(1).unwrap();
        assert_eq!(false, check_type1_validity(&second));
    }

    #[test]
    fn check_type2_validity_test() {
        let lines = vec!["1-3 a: abcde", "1-3 b: cdefg", "2-9 c: ccccccccc"].iter().map(|x| x.to_string()).collect();
        let password_entries = parse_file(&lines);

        let first = password_entries.get(0).unwrap();
        assert_eq!(true, check_type2_validity(&first));

        let second = password_entries.get(1).unwrap();
        assert_eq!(false, check_type2_validity(&second));

        let third = password_entries.get(2).unwrap();
        assert_eq!(false, check_type2_validity(&third));
    }

    #[test]
    fn part1_test() {
        let lines = vec!["1-3 a: abcde", "1-3 b: cdefg", "2-9 c: ccccccccc"].iter().map(|x| x.to_string()).collect();
        assert_eq!(2, part1(&lines));
    }

    #[test]
    fn part2_test() {
        let lines = vec!["1-3 a: abcde", "1-3 b: cdefg", "2-9 c: ccccccccc"].iter().map(|x| x.to_string()).collect();
        assert_eq!(1, part2(&lines));
    }
}