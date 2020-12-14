use std::path::Path as Path;
use std::time::Instant;

use crate::common::io;

fn move_ship(x: f32, y: f32, m: f32, operator: char, value: f32) -> (f32, f32, f32) {
    let mut new_x = x;
    let mut new_y = y;
    let mut new_m = m;

    match operator {
        'E' => {
            new_x += value.abs();
        },
        'W' => {
            new_x -= value.abs();
        },
        'N' => {
            new_y += value.abs();
        },
        'S' => {
            new_y -= value.abs();
        },
        'L' => {
            new_m += value.abs();
        },
        'R' => {
            new_m -= value.abs();
        },
        'F' => {
            new_x += value.abs() * (m * 0.0174533).cos();
            new_y += value.abs() * (m * 0.0174533).sin();
        },
        _ => {}
    }
    return (new_x, new_y, new_m);
}

fn use_waypoint_navigation(x: f32, y: f32, way_x: f32, way_y: f32, operator: char, value: f32) -> (f32, f32, f32, f32) {
    let mut new_x = x;
    let mut new_y = y;
    let mut new_way_x = way_x;
    let mut new_way_y = way_y;

    match operator {
        'E' => {
            new_way_x += value.abs();
        },
        'W' => {
            new_way_x -= value.abs();
        },
        'N' => {
            new_way_y += value.abs();
        },
        'S' => {
            new_way_y -= value.abs();
        },
        'L' => {
            new_way_x = (way_x * (value.abs() * 0.0174533).cos()) - (way_y * (value.abs() * 0.0174533).sin());
            new_way_y = (way_x * (value.abs() * 0.0174533).sin()) + (way_y * (value.abs() * 0.0174533).cos());
        },
        'R' => {
            new_way_x = (way_x * ( -1.0 * value.abs() * 0.0174533).cos()) - (way_y * (-1.0 * value.abs() * 0.0174533).sin());
            new_way_y = (way_x * ( -1.0 * value.abs() * 0.0174533).sin()) + (way_y * ( -1.0 * value.abs() * 0.0174533).cos());
        },
        'F' => {
            new_x += value.abs() * way_x;
            new_y += value.abs() * way_y;
        },
        _ => {}
    }
    return (new_x, new_y, new_way_x, new_way_y);
}

fn part1(lines: &Vec<String>) -> f32 {
    log::info!("Running Part 1");
    let mut x: f32 = 0.0;
    let mut y: f32 = 0.0;
    let mut m: f32 = 0.0;

    for line in lines {
        let chars: Vec<char> = line.chars().collect();
        let operator  = chars[0];
        let operrand: String = chars.into_iter().skip(1).collect();
        let value: i32 = operrand.parse().unwrap();
        log::debug!("Parsed operator {} and value {}", operator, value);
        let result = move_ship(x, y, m, operator, value as f32);
        x = result.0;
        y = result.1;
        m = result.2;
        log::debug!("New x {} y {} heading {}", x, y, m);
    }
    log::info!("manhatten distance travelled is {}", x.abs().round() + y.abs().round());
    return x.abs().round() + y.abs().round();
}

fn part2(lines: &Vec<String>) -> f32 {
    log::info!("Running Part 2");
    let mut waypoint_x: f32 = 10.0;
    let mut waypoint_y: f32 = 1.0;
    let mut ship_x: f32 = 0.0;
    let mut ship_y: f32 = 0.0;

    for line in lines {
        let chars: Vec<char> = line.chars().collect();
        let operator  = chars[0];
        let operrand: String = chars.into_iter().skip(1).collect();
        let value: i32 = operrand.parse().unwrap();
        log::debug!("Parsed operator {} and value {}", operator, value);
        let result = use_waypoint_navigation(ship_x, ship_y, waypoint_x, waypoint_y, operator, value as f32);
        ship_x = result.0;
        ship_y = result.1;
        waypoint_x = result.2;
        waypoint_y = result.3;
        log::debug!("Ship x {} y {} Waypoint x {} y {}", ship_x, ship_y, waypoint_x, waypoint_y);
    }
    log::info!("manhatten distance travelled is {}", ship_x.abs().round() + ship_y.abs().round());
    return ship_x.abs().round() + ship_y.abs().round();
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
            "F10",
            "N3",
            "F7",
            "R90",
            "F11"
        ].iter().map(|x| x.to_string()).collect();

        assert_eq!(25.0, part1(&instructions));
    }

    #[test]
    fn part2_test() {
        let instructions: Vec<String> = vec![
            "F10",
            "N3",
            "F7",
            "R90",
            "F11"
        ].iter().map(|x| x.to_string()).collect();

        assert_eq!(286.0, part2(&instructions));
    }
}