use std::path::Path as Path;

use crate::common::io;

fn generate_seat_id(guid: &String) -> i32 {
    let row_string = guid.chars().take(7);
    let seat_string = guid.chars().skip(7).take(3);

    let mut i = 0;
    let mut j = 127;

    for character in row_string {
        if character == 'F' {
            j = (((i + j) as f32) / 2.0).floor() as i32;
        } else if character == 'B' {
            i = (((i + j) as f32) / 2.0).ceil() as i32;
        } else {
            return -1;
        }
    }
    
    let row = (((i + j) as f32) / 2.0).floor() as i32;

    i = 0;
    j = 7;

    for character in seat_string {
        if character == 'L' {
            j = (((i + j) as f32) / 2.0).floor() as i32;
        } else if character == 'R' {
            i = (((i + j) as f32) / 2.0).ceil() as i32;
        } else {
            return -1;
        }
    }

    let seat = (((i + j) as f32) / 2.0).floor() as i32;
    return row * 8 + seat;
}

fn part1(lines: &Vec<String>) {
    let mut max = -1;

    for line in lines {
        let number = generate_seat_id(&line);
        if number > max {
            max = number;
        }
    }

    log::info!("The max seat number seen is {}", max);
}

fn part2(lines: &Vec<String>) {
    let mut seats: Vec<i32> = Vec::new();

    for line in lines {
        seats.push(generate_seat_id(&line));
    }

    seats.sort();

    for i in 0..seats.len() {
        if seats[i] + 1 != seats[i+1] {
            log::info!("The missing seat number is {}", seats[i] + 1);
            return;
        }
    }
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
    fn generate_seat_id_test() {
        assert_eq!(357, generate_seat_id(&"FBFBBFFRLR".to_string()));
        assert_eq!(567, generate_seat_id(&"BFFFBBFRRR".to_string()));
        assert_eq!(119, generate_seat_id(&"FFFBBBFRRR".to_string()));
        assert_eq!(820, generate_seat_id(&"BBFFBBFRLL".to_string()));
    }
}