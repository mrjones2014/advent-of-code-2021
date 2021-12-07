use std::error::Error;
use utils::input_parser;

fn compute_min_fuel_movement<F>(inputs: &[i32], compute_fuel_usage: F) -> i32
where
    F: Fn(i32, i32) -> i32,
{
    (*inputs.iter().min().unwrap()..(*inputs.iter().max().unwrap() + 1))
        .into_iter()
        .map(|movement| {
            inputs
                .iter()
                .map(|input| compute_fuel_usage(movement, *input))
                .sum::<i32>()
        })
        .min()
        .expect("No minimum value found")
}

fn main() -> Result<(), Box<dyn Error>> {
    let inputs = input_parser::parse("puzzle7")[0]
        .split(',')
        .map(|s| s.parse().expect("Failed to parse input to i32"))
        .collect::<Vec<i32>>();
    println!(
        "Part 1: {}",
        compute_min_fuel_movement(&inputs, |movement, input| { (input - movement).abs() })
    );
    println!(
        "Part 2: {}",
        compute_min_fuel_movement(&inputs, |movement, input| {
            let n = (input - movement).abs();
            (n * (n + 1)) / 2
        })
    );
    Ok(())
}
