use std::env;

mod days;
mod common;
#[macro_use]
extern crate log;

fn main() {
    env_logger::init();
    for day in env::args().skip(1) {
        log::info!("Running -> {}", day);
        match day.as_ref() {
            "day1" => days::day1::run("./inputs/day1"),
            _ => println!("Day not built so far!"),
        }
    }
}