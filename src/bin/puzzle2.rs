use std::{error::Error, fmt::Display};

use utils::input_parser;

enum Direction {
    FORWARD,
    DOWN,
    UP,
}

impl From<&String> for Direction {
    fn from(input: &String) -> Self {
        if input.contains("forward") {
            return Direction::FORWARD;
        }

        if input.contains("down") {
            return Direction::DOWN;
        }

        if input.contains("up") {
            return Direction::UP;
        }

        panic!("Unsupported direction: {}", input);
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::FORWARD => write!(f, "forward"),
            Direction::DOWN => write!(f, "down"),
            Direction::UP => write!(f, "up"),
        }
    }
}

fn part_1(input: &Vec<String>) -> Result<(), Box<dyn Error>> {
    let mut coordinates = (0, 0); // horizontal pos, depth
    for movement in input.iter() {
        let direction = Direction::from(movement);
        let value: i32 = movement
            .replace(format!("{}", direction).as_str(), "")
            .trim()
            .parse()?;
        match direction {
            Direction::FORWARD => coordinates = (coordinates.0 + value, coordinates.1),
            Direction::DOWN => coordinates = (coordinates.0, coordinates.1 + value),
            Direction::UP => coordinates = (coordinates.0, coordinates.1 - value),
        }
    }

    println!("Part 1\n=======");
    println!(
        "Coordinates: horizontal: {}, depth: {}",
        coordinates.0, coordinates.1
    );
    println!("horizontal * depth = {}", coordinates.0 * coordinates.1);

    Ok(())
}

fn part_2(input: &Vec<String>) -> Result<(), Box<dyn Error>> {
    let mut coordinates = (0, 0, 0); // horizontal pos, depth, aim
    for movement in input.iter() {
        let direction = Direction::from(movement);
        let value: i32 = movement
            .replace(format!("{}", direction).as_str(), "")
            .trim()
            .parse()?;
        match direction {
            Direction::FORWARD => {
                coordinates = (
                    coordinates.0 + value,
                    coordinates.1 + (coordinates.2 * value),
                    coordinates.2,
                )
            }
            Direction::DOWN => coordinates = (coordinates.0, coordinates.1, coordinates.2 + value),
            Direction::UP => coordinates = (coordinates.0, coordinates.1, coordinates.2 - value),
        }
    }

    println!("Part 2\n=======");
    println!(
        "Coordinates: horizontal: {}, depth: {}, aim: {}",
        coordinates.0, coordinates.1, coordinates.2
    );
    println!("horizontal * depth = {}", coordinates.0 * coordinates.1);

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = input_parser::parse("puzzle2");
    part_1(&input)?;
    println!("");
    part_2(&input)?;

    Ok(())
}
