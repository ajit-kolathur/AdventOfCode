use std::path::Path as Path;
use std::time::Instant;
use num::integer;

use crate::common::io;

fn part1(timestamp: i32, busses: &Vec<i32>) -> f32 {
    log::info!("Running Part 1");
    let mut min_arrival_time = i32::MAX;
    let mut earliest_bus = -1;

    for bus in busses {
        let arrival_time = bus - (timestamp%bus);
        log::debug!("Min arrival time {} for bus {}", arrival_time, bus);
        if arrival_time >= 0 && arrival_time < min_arrival_time {
            min_arrival_time = arrival_time;
            earliest_bus = *bus;
        }
    }
    log::debug!("Arrival time in {}, Arriving bus {}, product {}", min_arrival_time, earliest_bus, min_arrival_time as f32 * earliest_bus as f32);
    return earliest_bus as f32 * min_arrival_time as f32;
}

fn part2(busses_list: &Vec<&str>) -> i128 {
    log::info!("Running Part 2");
    let in_service_busses: Vec<i32> = busses_list
                            .iter()
                            .filter(|x| x.to_string() != "x")
                            .map(|x| x.parse::<i32>().unwrap())
                            .collect();
    let mut product: i64 = 1;
    for bus in in_service_busses {
        product *= bus as i64;
    }
    
    log::debug!("LCM {}", product);

    let mut busses: Vec<(i32, i32)> = Vec::new();

    for i in 0..busses_list.len() {
        if busses_list[i] != "x" {
            let number: i32 = busses_list[i].parse().unwrap();
            let index = i as i32;
            log::debug!("(x+{}) mod {} = 0", index, number);
            busses.push((number, index));
        }
    }

    busses.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    busses.reverse();
    log::debug!("Sorted list {:?}", busses);

    // If we were to approach this by adding one bus to the mix at a time,
    // the recurrence period should by the LCM of busses ids
    let mut count = 0;
    let mut occurance: i128 = 0;
    let mut period: i128 = 1;

    while count < busses.len() {
        while (occurance + busses[count].1 as i128) % busses[count].0 as i128 != 0 {
            occurance += period;
        }
        period *= busses[count].0 as i128;
        log::debug!("First occurance satisfying {} rules is {}, period {}", count+1, occurance, period);
        count += 1;
    }
    log::info!("First occurance satisfying all rules is {}", occurance);
    return occurance;
}

fn is_set_coprime(numbers: &Vec<i32>) -> bool {
    for number in numbers {
        for other in numbers {
            if number != other && integer::gcd(*number, *other) != 1 {
                return false;
            }
        }
    }
    return true;
}

pub fn run(filename: impl AsRef<Path>) {
    let lines: Vec<String> = io::lines_from_file(filename);
    let timestamp: i32 = lines[0].parse().unwrap();
    let busses_list: Vec<&str> = lines[1].split(',').collect();
    let in_service_busses = busses_list
                            .iter()
                            .filter(|x| x.to_string() != "x")
                            .map(|x| x.parse::<i32>().unwrap())
                            .collect();
    let now = Instant::now();
    part1(timestamp, &in_service_busses);
    log::info!("Part 1: {}us", now.elapsed().as_micros());
    
    if is_set_coprime(&in_service_busses) {
        part2(&busses_list);
        log::info!("Part 1 + 2: {}us", now.elapsed().as_micros());
    } else {
        panic!("If the bus set is not coprime we cannot solve this, crash and burn");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let busses: Vec<i32> = vec![7,13,59,31,19];
        assert_eq!(295.0, part1(939, &busses));
    }

    #[test]
    fn part2_test() {
        let busses: Vec<&str> = vec!["7","13","x","x","59","x","31","19"];
        assert_eq!(1068781, part2(&busses));
    }
}