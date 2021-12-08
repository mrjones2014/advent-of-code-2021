use std::{collections::HashMap, error::Error, str::FromStr};

use utils::input_parser;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum SevenSegment {
    Zero = 0,
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
}

impl SevenSegment {
    pub fn num_segments(&self) -> usize {
        match self {
            SevenSegment::Zero => 6,
            SevenSegment::One => 2,
            SevenSegment::Two => 5,
            SevenSegment::Three => 5,
            SevenSegment::Four => 4,
            SevenSegment::Five => 5,
            SevenSegment::Six => 6,
            SevenSegment::Seven => 3,
            SevenSegment::Eight => 7,
            SevenSegment::Nine => 6,
        }
    }
}

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
        .flat_map(|signal| signal.output.to_owned())
        .filter(|output| {
            let output_len = output.len();

            output_len == SevenSegment::One.num_segments()
                || output_len == SevenSegment::Four.num_segments()
                || output_len == SevenSegment::Seven.num_segments()
                || output_len == SevenSegment::Eight.num_segments()
        })
        .count()
}

fn main() -> Result<(), Box<dyn Error>> {
    let inputs: Vec<Signal> = input_parser::parse("puzzle8")
        .iter()
        .map(|s| Signal::from_str(s).expect("Failed to parse signal"))
        .collect();

    println!("Part 1: {}", part_1(&inputs));

    Ok(())
}
