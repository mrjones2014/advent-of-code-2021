use std::error::Error;

use utils::input_parser;

fn main() -> Result<(), Box<dyn Error>> {
    let inputs = input_parser::parse("puzzle7")[0]
        .split(",")
        .map(|s| s.parse().expect("Failed to parse input to u16"))
        .collect::<Vec<i32>>();
    let mean = inputs.iter().sum::<i32>() / inputs.len() as i32;
    let min_movement_position = (*inputs.iter().min().unwrap()..(mean + 1))
        .into_iter()
        .map(|movement| {
            inputs
                .iter()
                .map(|input| (input - movement).abs())
                .sum::<i32>()
        })
        .min()
        .expect("No minimum value found");
    println!("Part 1: {}", min_movement_position);
    Ok(())
}
