use std::path::Path as Path;
use regex::RegexSet;
use regex::Regex;

use crate::common::io;
use crate::common::convertor::chunk_parts as chunk_parts;

fn check_all_rules_are_obeyed(chunked_lines: &Vec<String>, regex_set: &RegexSet) -> i32 {
    let mut count = 0;
    for line in chunked_lines {
        let matches: Vec<_> = regex_set.matches(line).into_iter().collect();
        if matches.len() == regex_set.len() {
            count += 1;
        }
    }
    return count;
}

fn part1(lines: &Vec<String>) -> i32 {
    log::info!("Running Part 1");
    let regex_set = RegexSet::new(&[
        r"byr:",
        r"iyr:",
        r"eyr:",
        r"hgt:",
        r"hcl:",
        r"ecl:",
        r"pid:",
    ]).unwrap();
    
    let mut count = 0;
    for line in lines {
        let matches: Vec<_> = regex_set.matches(line).into_iter().collect();
        if matches.len() == regex_set.len() {
            count += 1;
        }
    }

    log::info!("Found {} valid entries", count);
    return count;
}

fn validate_date_range_regex(regex: &Regex, line: &String, min: i16, max: i16) -> bool {
    let cap_grp = regex.captures(line);
    if cap_grp.is_some() {
        let cap = cap_grp.unwrap();
        if cap.len() > 0 {
            let year: i16 = cap.get(1).unwrap().as_str().parse().unwrap();
            return year >= min && year <= max;
        }
    }
    return false;
}

fn validate_height_range_regex(regex: &Regex, line: &String) -> bool {
    let cap_grp = regex.captures(line);
    if cap_grp.is_some() {
        let cap = cap_grp.unwrap();
        if cap.len() > 0 {
            let height = cap.get(1).unwrap().as_str();
            if height.contains("cm") {
                let height_cm: i16 = height.replace("cm", "").parse().unwrap();
                return height_cm >= 150 && height_cm <= 193;
            }

            if height.contains("in") {
                let height_cm: i16 = height.replace("in", "").parse().unwrap();
                return height_cm >= 59 && height_cm <= 76;
            }
        }
    }
    return false;
}

fn verify_hex_regex(regex: &Regex, line: &String) -> bool {
    let cap_grp = regex.captures(line);
    if cap_grp.is_some() {
        let cap = cap_grp.unwrap();
        if cap.len() > 0 {
            let length = cap.get(1).unwrap().as_str().len();
            return length == 6;
        }
    }
    return false;
}

fn verify_eye_color_regex(regex: &Regex, line: &String) -> bool {
    let cap_grp = regex.captures(line);
    let colors = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
    if cap_grp.is_some() {
        let cap = cap_grp.unwrap();
        if cap.len() > 0 {
            let color = cap.get(1).unwrap().as_str();
            return colors.iter().any(|&c| c == color);
        }
    }
    return false;
}

fn verify_pid_regex(regex: &Regex, line: &String) -> bool {
    let cap_grp = regex.captures(line);
    if cap_grp.is_some() {
        let cap = cap_grp.unwrap();
        if cap.len() > 0 {
            let pid = cap.get(1).unwrap().as_str();
            return pid.len() == 9;
        }
    }
    return false;
}

fn part2(lines: &Vec<String>) -> i32 {
    log::info!("Running Part 2");
    let byr = Regex::new(r"byr:(\d{4})").unwrap();
    let iyr = Regex::new(r"iyr:(\d{4})").unwrap();
    let eyr = Regex::new(r"eyr:(\d{4})").unwrap();
    let hgt = Regex::new(r"hgt:([0-9]+[a-z]*)").unwrap();
    let hcl = Regex::new(r"hcl:#([0-9a-f]*)").unwrap();
    let ecl = Regex::new(r"ecl:(\w{3})").unwrap();
    let pid = Regex::new(r"pid:(\d{9})").unwrap();
    
    let mut count = 0;
    for line in lines {
        log::debug!("Processing line {}", line);
        if validate_date_range_regex(&byr, line, 1920, 2002) &&
           validate_date_range_regex(&iyr, line, 2010, 2020) &&
           validate_date_range_regex(&eyr, line, 2020, 2030) &&
           validate_height_range_regex(&hgt, line) &&
           verify_hex_regex(&hcl, line) &&
           verify_eye_color_regex(&ecl, line) &&
           verify_pid_regex(&pid, line) {
            count += 1;
           }

    }
    log::info!("Found {} valid entries", count);
    return count;
}

pub fn run(filename: impl AsRef<Path>) {
    let lines = io::lines_from_file(filename);
    let chunks = chunk_parts(&lines);

    part1(&chunks);
    part2(&chunks);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let lines = vec![        
            "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd",
            "byr:1937 iyr:2017 cid:147 hgt:183cm",
            "",
            "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884",
            "hcl:#cfa07d byr:1929",
            "",
            "hcl:#ae17e1 iyr:2013",
            "eyr:2024",
            "ecl:brn pid:760753108 byr:1931",
            "hgt:179cm",
            "",
            "hcl:#cfa07d eyr:2025 pid:166559648",
            "iyr:2011 ecl:brn hgt:59in"]
            .iter().map(|x| x.to_string()).collect();
        
        let chunked_lines = chunk_parts(&lines);

        assert_eq!(2, part1(&chunked_lines));
    }

    #[test]
    fn regex_test() {
        let byr = Regex::new(r"byr:(\d{4})").unwrap();
        let hgt = Regex::new(r"hgt:([0-9]+[a-z]*)").unwrap();
        let hcl = Regex::new(r"hcl:#([0-9a-f]*)").unwrap();
        let ecl = Regex::new(r"ecl:(\w{3})").unwrap();
        let pid = Regex::new(r"pid:([0-9]+)").unwrap();
        
        assert_eq!(true, validate_date_range_regex(&byr, &"byr:2002".to_string(), 1920, 2002));
        assert_eq!(false, validate_date_range_regex(&byr, &"byr:2003".to_string(), 1920, 2002));
        assert_eq!(true, validate_height_range_regex(&hgt, &"hgt:60in".to_string()));
        assert_eq!(true, validate_height_range_regex(&hgt, &"hgt:190cm".to_string()));
        assert_eq!(false, validate_height_range_regex(&hgt, &"hgt:190in".to_string()));
        assert_eq!(false, validate_height_range_regex(&hgt, &"hgt:190".to_string()));
        assert_eq!(true, verify_hex_regex(&hcl, &"hcl:#123abc".to_string()));
        assert_eq!(false, verify_hex_regex(&hcl, &"hcl:#123abz".to_string()));
        assert_eq!(false, verify_hex_regex(&hcl, &"hcl:123abc".to_string()));
        assert_eq!(true, verify_eye_color_regex(&ecl, &"ecl:brn".to_string()));
        assert_eq!(false, verify_eye_color_regex(&ecl, &"ecl:wat".to_string()));
        assert_eq!(true, verify_pid_regex(&pid, &"pid:000000001".to_string()));
        assert_eq!(false, verify_pid_regex(&pid, &"pid:0123456789".to_string()));
    }

    #[test]
    fn part2_test_invalid() {
        let lines = vec![
            "eyr:1972 cid:100",
            "hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926",
            "",
            "iyr:2019",
            "hcl:#602927 eyr:1967 hgt:170cm",
            "ecl:grn pid:012533040 byr:1946",
            "",
            "hcl:dab227 iyr:2012",
            "ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277",
            "",
            "hgt:59cm ecl:zzz",
            "eyr:2038 hcl:74454a iyr:2023",
            "pid:3556412378 byr:2007"
        ]
        .iter().map(|x| x.to_string()).collect();

        let chunked_lines = chunk_parts(&lines);
        assert_eq!(0, part2(&chunked_lines));
    }

    #[test]
    fn part2_test_valid() {
        let lines = vec![
            "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980",
            "hcl:#623a2f",
            "",
            "eyr:2029 ecl:blu cid:129 byr:1989",
            "iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm",
            "",
            "hcl:#888785",
            "hgt:164cm byr:2001 iyr:2015 cid:88",
            "pid:545766238 ecl:hzl",
            "eyr:2022",
            "",
            "iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719"
        ]
        .iter().map(|x| x.to_string()).collect();

        let chunked_lines = chunk_parts(&lines);
        assert_eq!(4, part2(&chunked_lines));
    }
}