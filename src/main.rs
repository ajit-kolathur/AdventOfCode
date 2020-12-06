use std::env;

mod days;
mod common;
extern crate log;

fn main() {
    env_logger::init();
    for day in env::args().skip(1) {
        log::info!("Running -> {}", day);
        match day.as_ref() {
            "day1" => days::day1::run("./inputs/day1"),
            "day2" => days::day2::run("./inputs/day2"),
            "day3" => days::day3::run("./inputs/day3"),
            "day4" => days::day4::run("./inputs/day4"),
            "day5" => days::day5::run("./inputs/day5"),
            "day6" => days::day6::run("./inputs/day6"),
            _ => println!("Day not built so far!"),
        }
    }
}