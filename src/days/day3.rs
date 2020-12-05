use std::path::Path as Path;

use crate::common::io;

fn traverse_matrix(lines: &Vec<String>, x_move: i8, y_move: i8) -> i32 {
    let mut x: i32 = 0;
    let mut y: i32 = 0;

    let x_max = lines.iter().next().unwrap().len() as i32;
    let y_max = lines.len() as i32;

    let mut count = 0;

    while y < y_max {
        let entry = lines[y as usize].chars().nth(x as usize).unwrap();
        if entry == '#' {
            count += 1;
        }

        x = (x + x_move as i32) % x_max;
        y += y_move as i32;
    }

    return count;
}

fn part1(matrix: &Vec<String>) -> i32 {
    log::info!("Running Part 1");
    let count = traverse_matrix(matrix, 3, 1);
    log::info!("Found {} trees on the way down", count);
    return count;
}

fn part2(matrix: &Vec<String>) -> i32 {
    log::info!("Running Part 1");
    let mut result = 1;
    result *= traverse_matrix(matrix, 1, 1);
    result *= traverse_matrix(matrix, 3, 1);
    result *= traverse_matrix(matrix, 5, 1);
    result *= traverse_matrix(matrix, 7, 1);
    result *= traverse_matrix(matrix, 1, 2);
    log::info!("Found {} trees on the way down", result);
    return result;
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
        let matrix = 
            vec!["..##.......",
                 "#...#...#..",
                 ".#....#..#.",
                 "..#.#...#.#",
                 ".#...##..#.",
                 "..#.##.....",
                 ".#.#.#....#",
                 ".#........#",
                 "#.##...#...",
                 "#...##....#",
                 ".#..#...#.#"]
            .iter().map(|x| x.to_string()).collect();
        assert_eq!(7, part1(&matrix));
    }

    #[test]
    fn part2_test() {
        let matrix = 
            vec!["..##.......",
                 "#...#...#..",
                 ".#....#..#.",
                 "..#.#...#.#",
                 ".#...##..#.",
                 "..#.##.....",
                 ".#.#.#....#",
                 ".#........#",
                 "#.##...#...",
                 "#...##....#",
                 ".#..#...#.#"]
            .iter().map(|x| x.to_string()).collect();
        assert_eq!(336, part2(&matrix));
    }
}