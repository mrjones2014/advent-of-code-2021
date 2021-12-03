use std::{error::Error, str::FromStr};

use utils::input_parser;

enum Direction {
    Forward(i32),
    Down(i32),
    Up(i32),
}

impl FromStr for Direction {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (direction, value_str) = s.trim().split_once(' ').expect("Missing value in input");
        let value: i32 = value_str.parse()?;
        Ok(match direction {
            "up" => Self::Up(value),
            "down" => Self::Down(value),
            "forward" => Self::Forward(value),
            _ => panic!("Invalid input"),
        })
    }
}

fn part_1(input: &[String]) -> Result<i32, Box<dyn Error>> {
    let directions: Vec<Direction> = input
        .iter()
        .map(|line| Direction::from_str(line).expect("Invalid input"))
        .collect();
    let (mut x, mut z) = (0, 0);
    for direction in directions {
        match direction {
            Direction::Up(value) => z -= value,
            Direction::Down(value) => z += value,
            Direction::Forward(value) => x += value,
        }
    }

    Ok(x * z)
}

fn part_2(input: &[String]) -> Result<i32, Box<dyn Error>> {
    let directions: Vec<Direction> = input
        .iter()
        .map(|line| Direction::from_str(line).expect("Invalid input"))
        .collect();
    let (mut x, mut z, mut aim) = (0, 0, 0);
    for direction in directions {
        match direction {
            Direction::Up(n) => aim -= n,
            Direction::Down(n) => aim += n,
            Direction::Forward(n) => {
                x += n;
                z += aim * n;
            }
        }
    }

    Ok(x * z)
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = input_parser::parse("puzzle2");
    println!("Part 1: {}", part_1(&input)?);
    println!("Part 2: {}", part_2(&input)?);
    Ok(())
}
