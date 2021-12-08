use std::{error::Error, str::FromStr};

use utils::input_parser;

struct Signal {
    input: Vec<String>,
    output: Vec<String>,
}

impl FromStr for Signal {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (raw_input, raw_output) = s.split_once('|').expect("No delimeter found.");
        let input = raw_input.trim().split(' ').map(|s| s.to_owned()).collect();
        let output = raw_output.trim().split(' ').map(|s| s.to_owned()).collect();
        Ok(Signal { input, output })
    }
}

fn part_1(signals: &[Signal]) -> usize {
    signals
        .iter()
        .map(|signal| {
            signal
                .output
                .iter()
                .filter(|output| {
                    output.len() == 2 // digit 1
                || output.len() == 4 // digit 4
                || output.len() == 3 // digit 7
                || output.len() == 7 // digit 8
                })
                .count()
        })
        .sum()
}

fn main() -> Result<(), Box<dyn Error>> {
    let inputs: Vec<Signal> = input_parser::parse("puzzle8")
        .iter()
        .map(|s| Signal::from_str(s).expect("Failed to parse signal"))
        .collect();

    println!("Part 1: {}", part_1(&inputs));

    Ok(())
}
