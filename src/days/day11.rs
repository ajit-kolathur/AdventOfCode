use std::path::Path as Path;
use std::time::Instant;

use crate::common::io;

fn generate_neighbors(x: i16, y: i16, x_max: i16, y_max: i16) -> Vec<(usize,usize)>{
    let neighbors: Vec<(usize, usize)> = vec![
        (x-1,y),
        (x-1,y-1),
        (x,y-1),
        (x+1,y-1),
        (x+1,y),
        (x+1,y+1),
        (x,y+1),
        (x-1,y+1)
    ].iter()
    .filter(|&(i,j)| *i >= 0 && *j >= 0 && *i < x_max && *j < y_max)
    .map(|entry| (entry.0 as usize, entry.1 as usize))
    .collect::<Vec<_>>();
    return neighbors;
}

fn generate_neighbor_seats(x: i16, y: i16, seating: &Vec<Vec<char>>) -> Vec<(usize,usize)>{
    let mut temp_x: i16;
    let mut temp_y: i16;

    let mut result: Vec<(usize, usize)> = Vec::new();

    // get neighbors to the east
    temp_x = x + 1;
    temp_y = y;
    while temp_x >= 0 &&
          temp_y >= 0 &&
          temp_x < seating.len() as i16 &&
          temp_y < seating.first().unwrap().len() as i16 
    {
        log::trace!("Looking at {},{}", temp_x, temp_y);
        if seating[temp_x as usize][temp_y as usize] != '.' {
            result.push((temp_x as usize, temp_y as usize));
            break;
        }
        temp_x += 1;
    }
    // get neighbors to the west
    temp_x = x - 1;
    temp_y = y;
    while temp_x >= 0 &&
          temp_y >= 0 &&
          temp_x < seating.len() as i16 &&
          temp_y < seating.first().unwrap().len() as i16 
    {
        log::trace!("Looking at {},{}", temp_x, temp_y);
        if seating[temp_x as usize][temp_y as usize] != '.' {
            result.push((temp_x as usize, temp_y as usize));
            break;
        }
        temp_x -= 1;
    }
    // get neighbors to the north
    temp_x = x;
    temp_y = y - 1;
    while temp_x >= 0 &&
          temp_y >= 0 &&
          temp_x < seating.len() as i16 &&
          temp_y < seating.first().unwrap().len() as i16 
    {
        log::trace!("Looking at {},{}", temp_x, temp_y);
        if seating[temp_x as usize][temp_y as usize] != '.' {
            result.push((temp_x as usize, temp_y as usize));
            break;
        }
        temp_y -= 1;
    }
    // get neighbors to the south
    temp_x = x;
    temp_y = y+1;
    while temp_x >= 0 &&
          temp_y >= 0 &&
          temp_x < seating.len() as i16 &&
          temp_y < seating.first().unwrap().len() as i16 
    {
        log::trace!("Looking at {},{}", temp_x, temp_y);
        if seating[temp_x as usize][temp_y as usize] != '.' {
            result.push((temp_x as usize, temp_y as usize));
            break;
        }
        temp_y += 1;
    }
    // get neighbors to the north east
    temp_x = x + 1;
    temp_y = y - 1;
    while temp_x >= 0 &&
          temp_y >= 0 &&
          temp_x < seating.len() as i16 &&
          temp_y < seating.first().unwrap().len() as i16 
    {
        log::trace!("Looking at {},{}", temp_x, temp_y);
        if seating[temp_x as usize][temp_y as usize] != '.' {
            result.push((temp_x as usize, temp_y as usize));
            break;
        }
        temp_x += 1;
        temp_y -= 1;
    }
    // get neighbors to the north west
    temp_x = x - 1;
    temp_y = y - 1;
    while temp_x >= 0 &&
          temp_y >= 0 &&
          temp_x < seating.len() as i16 &&
          temp_y < seating.first().unwrap().len() as i16 
    {
        log::trace!("Looking at {},{}", temp_x, temp_y);
        if seating[temp_x as usize][temp_y as usize] != '.' {
            result.push((temp_x as usize, temp_y as usize));
            break;
        }
        temp_x -= 1;
        temp_y -= 1;
    }
    // get neighbors to the south east
    temp_x = x + 1;
    temp_y = y + 1;
    while temp_x >= 0 &&
          temp_y >= 0 &&
          temp_x < seating.len() as i16 &&
          temp_y < seating.first().unwrap().len() as i16 
    {
        log::trace!("Looking at {},{}", temp_x, temp_y);
        if seating[temp_x as usize][temp_y as usize] != '.' {
            result.push((temp_x as usize, temp_y as usize));
            break;
        }
        temp_x += 1;
        temp_y += 1;
    }
    // get neighbors to the south west
    temp_x = x - 1;
    temp_y = y + 1;
    while temp_x >= 0 &&
          temp_y >= 0 &&
          temp_x < seating.len() as i16 &&
          temp_y < seating.first().unwrap().len() as i16 
    {
        log::trace!("Looking at {},{}", temp_x, temp_y);
        if seating[temp_x as usize][temp_y as usize] != '.' {
            result.push((temp_x as usize, temp_y as usize));
            break;
        }
        temp_x -= 1;
        temp_y += 1;
    }

    return result;
}

fn fill_seats(seating: &Vec<Vec<char>>) -> (i32, Vec<Vec<char>>) {
    let mut seats_changed = 0;
    let mut modified_seats = vec![vec!['.'; seating.first().unwrap().len()]; seating.len()];

    for row in 0..seating.len() {
        for column in 0..seating[row].len() {
            let seat = seating[row][column];
            modified_seats[row][column] = seat;

            if seat != '.' {
                let mut occupied_count = 0;
                for entry in generate_neighbors(row as i16, column as i16, seating.len() as i16, seating[row].len() as i16) {
                    if seating[entry.0][entry.1] == '#' {
                        occupied_count += 1;
                    }
                }

                if seat == 'L' && occupied_count == 0 {
                    modified_seats[row][column] = '#';
                    seats_changed += 1;
                    continue;
                }
                
                if seat == '#' && occupied_count >= 4 {
                    modified_seats[row][column] = 'L';
                    seats_changed += 1;
                    continue;
                }
            }
        }
    }
    return (seats_changed, modified_seats);
}

fn fill_seatsv2(seating: &Vec<Vec<char>>) -> (i32, Vec<Vec<char>>) {
    let mut seats_changed = 0;
    let mut modified_seats = vec![vec!['.'; seating.first().unwrap().len()]; seating.len()];

    for row in 0..seating.len() {
        for column in 0..seating[row].len() {
            let seat = seating[row][column];
            modified_seats[row][column] = seat;

            log::trace!("Looking at {} at {},{}", seat, row, column);

            if seat != '.' {
                let mut occupied_count = 0;
                for entry in generate_neighbor_seats(row as i16, column as i16, &seating) {
                    log::trace!("Looking at neighbor {},{}", entry.0, entry.1);
                    if seating[entry.0][entry.1] == '#' {
                        occupied_count += 1;
                    }
                }

                if seat == 'L' && occupied_count == 0 {
                    modified_seats[row][column] = '#';
                    seats_changed += 1;
                    continue;
                }
                
                if seat == '#' && occupied_count >= 5 {
                    modified_seats[row][column] = 'L';
                    seats_changed += 1;
                    continue;
                }
            }
        }
    }
    return (seats_changed, modified_seats);
}

fn part1(seating: &Vec<Vec<char>>) -> i32 {
    log::info!("Running Part 1");

    let mut seats_changed = -1;
    let mut modified_seating = seating.clone();

    while seats_changed != 0 {
        let result = fill_seats(&modified_seating);
        seats_changed = result.0;
        modified_seating = result.1;
    }

    let mut occupied_seats = 0;
    for row in 0..modified_seating.len() {
        for column in 0..modified_seating[row].len() { 
            if modified_seating[row][column] == '#' {
                occupied_seats += 1;
            }
        }
    }
    log::info!("Steady state seats occupied are {}", occupied_seats);
    return occupied_seats;
}

fn part2(seating: &Vec<Vec<char>>) -> i32 {
    log::info!("Running Part 2");
    let mut seats_changed = -1;
    let mut modified_seating = seating.clone();

    while seats_changed != 0 {
        let result = fill_seatsv2(&modified_seating);
        seats_changed = result.0;
        modified_seating = result.1;

        log::debug!("Printing state");
        for line in modified_seating.clone() {
            log::debug!("{:?}", line);
        }
    }

    let mut occupied_seats = 0;
    for row in 0..modified_seating.len() {
        for column in 0..modified_seating[row].len() { 
            if modified_seating[row][column] == '#' {
                occupied_seats += 1;
            }
        }
    }
    log::info!("Steady state seats occupied are {}", occupied_seats);
    return occupied_seats;
}

pub fn run(filename: impl AsRef<Path>) {
    let lines = io::lines_from_file(filename).iter().map(|x| x.chars().collect()).collect();
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
        let seat_map: Vec<Vec<char>> = vec![
            "L.LL.LL.LL",
            "LLLLLLL.LL",
            "L.L.L..L..",
            "LLLL.LL.LL",
            "L.LL.LL.LL",
            "L.LLLLL.LL",
            "..L.L.....",
            "LLLLLLLLLL",
            "L.LLLLLL.L",
            "L.LLLLL.LL"
        ].iter().map(|x| x.chars().collect()).collect();

        assert_eq!(37, part1(&seat_map))
    }

    #[test]
    fn part2_test() {
        let seat_map: Vec<Vec<char>> = vec![
            "L.LL.LL.LL",
            "LLLLLLL.LL",
            "L.L.L..L..",
            "LLLL.LL.LL",
            "L.LL.LL.LL",
            "L.LLLLL.LL",
            "..L.L.....",
            "LLLLLLLLLL",
            "L.LLLLLL.L",
            "L.LLLLL.LL"
        ].iter().map(|x| x.chars().collect()).collect();

        assert_eq!(26, part2(&seat_map))
    }

    #[test]
    fn generate_neighbor_seats_test() {
        let all_seat_map: Vec<Vec<char>> = vec![
            vec!['L','L','L'],
            vec!['L','L','L'],
            vec!['L','L','L'],
        ];
        assert_eq!(3, generate_neighbor_seats(0, 0, &all_seat_map).len());
        assert_eq!(5, generate_neighbor_seats(0, 1, &all_seat_map).len());
        assert_eq!(3, generate_neighbor_seats(0, 2, &all_seat_map).len());
        assert_eq!(5, generate_neighbor_seats(1, 0, &all_seat_map).len());
        assert_eq!(8, generate_neighbor_seats(1, 1, &all_seat_map).len());
        assert_eq!(5, generate_neighbor_seats(1, 2, &all_seat_map).len());
        assert_eq!(3, generate_neighbor_seats(2, 0, &all_seat_map).len());
        assert_eq!(5, generate_neighbor_seats(2, 1, &all_seat_map).len());
        assert_eq!(3, generate_neighbor_seats(2, 2, &all_seat_map).len());

        let no_seat_map: Vec<Vec<char>> = vec![
            vec!['.','.','.'],
            vec!['.','.','.'],
            vec!['.','.','.'],
        ];
        assert_eq!(0, generate_neighbor_seats(0, 0, &no_seat_map).len());
        assert_eq!(0, generate_neighbor_seats(0, 1, &no_seat_map).len());
        assert_eq!(0, generate_neighbor_seats(0, 2, &no_seat_map).len());
        assert_eq!(0, generate_neighbor_seats(1, 0, &no_seat_map).len());
        assert_eq!(0, generate_neighbor_seats(1, 1, &no_seat_map).len());
        assert_eq!(0, generate_neighbor_seats(1, 2, &no_seat_map).len());
        assert_eq!(0, generate_neighbor_seats(2, 0, &no_seat_map).len());
        assert_eq!(0, generate_neighbor_seats(2, 1, &no_seat_map).len());
        assert_eq!(0, generate_neighbor_seats(2, 2, &no_seat_map).len());

        let some_seat_map: Vec<Vec<char>> = vec![
            vec!['.','.','L'],
            vec!['.','L','.'],
            vec!['L','.','.'],
        ];
        assert_eq!(3, generate_neighbor_seats(0, 0, &some_seat_map).len());
        assert_eq!(2, generate_neighbor_seats(0, 1, &some_seat_map).len());
        assert_eq!(1, generate_neighbor_seats(0, 2, &some_seat_map).len());
        assert_eq!(2, generate_neighbor_seats(1, 0, &some_seat_map).len());
        assert_eq!(2, generate_neighbor_seats(1, 1, &some_seat_map).len());
        assert_eq!(2, generate_neighbor_seats(1, 2, &some_seat_map).len());
        assert_eq!(1, generate_neighbor_seats(2, 0, &some_seat_map).len());
        assert_eq!(2, generate_neighbor_seats(2, 1, &some_seat_map).len());
        assert_eq!(3, generate_neighbor_seats(2, 2, &some_seat_map).len());
    }
}