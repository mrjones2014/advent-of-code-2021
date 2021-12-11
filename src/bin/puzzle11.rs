use std::{collections::HashMap, error::Error};
use utils::input_parser;

const GRID_SIZE: usize = 10;
const MAX_ENERGY: u32 = 10;

fn parse_input() -> HashMap<(i32, i32), u32> {
    let rows = input_parser::parse("puzzle11");
    let mut octopi = HashMap::new();
    for (y, row) in rows.iter().enumerate() {
        for (x, octopus) in row.chars().enumerate() {
            octopi.insert(
                (y as i32, x as i32),
                octopus.to_digit(10).expect("Failed to parse octopus."),
            );
        }
    }

    octopi
}

fn flash(octopi: &mut HashMap<(i32, i32), u32>, y: i32, x: i32) -> u64 {
    let mut num_flashes = 1;
    for (delta_y, delta_x) in [
        (-1, 0),
        (1, 0),
        (0, -1),
        (0, 1),
        (-1, -1),
        (1, 1),
        (-1, 1),
        (1, -1),
    ] {
        if let Some(energy) = octopi.get_mut(&(y + delta_y, x + delta_x)) {
            *energy += 1;
            if *energy == MAX_ENERGY {
                num_flashes += flash(octopi, y + delta_y, x + delta_x);
            }
        }
    }
    num_flashes
}

fn part_1(octopi: &mut HashMap<(i32, i32), u32>) -> u64 {
    let mut flashes = 0;
    for _ in 0..(GRID_SIZE * GRID_SIZE) {
        for y in 0..GRID_SIZE {
            for x in 0..GRID_SIZE {
                if let Some(energy) = octopi.get_mut(&(y as i32, x as i32)) {
                    *energy += 1;
                    if *energy == MAX_ENERGY {
                        flashes += flash(octopi, y as i32, x as i32);
                    }
                }
            }
        }

        for y in 0..GRID_SIZE {
            for x in 0..GRID_SIZE {
                if let Some(energy) =
                    octopi.get_mut(&(y.try_into().unwrap(), x.try_into().unwrap()))
                {
                    if *energy >= MAX_ENERGY {
                        *energy = 0;
                    }
                }
            }
        }
    }

    flashes
}

fn part_2(octopi: &mut HashMap<(i32, i32), u32>) -> u64 {
    for i in 1.. {
        for y in 0..GRID_SIZE {
            for x in 0..GRID_SIZE {
                if let Some(energy) = octopi.get_mut(&(y as i32, x as i32)) {
                    *energy += 1;
                    if *energy == MAX_ENERGY {
                        flash(octopi, y as i32, x as i32);
                    }
                }
            }
        }

        let mut is_synchronized = true;
        for y in 0..GRID_SIZE {
            for x in 0..GRID_SIZE {
                if let Some(energy) = octopi.get_mut(&(y as i32, x as i32)) {
                    if *energy >= MAX_ENERGY {
                        *energy = 0;
                    } else {
                        is_synchronized = false;
                    }
                }
            }
        }

        if is_synchronized {
            return i;
        }
    }

    unreachable!();
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut part_1_octopi = parse_input();
    let mut part_2_octopi = part_1_octopi.clone();

    println!("Part 1: {}", part_1(&mut part_1_octopi));
    println!("Part 2: {}", part_2(&mut part_2_octopi));

    Ok(())
}
