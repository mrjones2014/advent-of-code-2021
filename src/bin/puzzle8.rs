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

fn sort_chars(s: &str) -> String {
    let mut chars = s.chars().collect::<Vec<char>>();
    chars.sort_unstable();
    chars.into_iter().collect()
}

impl FromStr for Signal {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (raw_input, raw_output) = s.split_once('|').expect("No delimeter found.");
        let input = raw_input.trim().split(' ').map(sort_chars).collect();
        let output = raw_output.trim().split(' ').map(sort_chars).collect();
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

fn part_2(signals: &[Signal]) -> u32 {
    let mut result = 0;

    for signal in signals {
        let mut char_frequency: HashMap<usize, Vec<String>> = HashMap::new();
        for word in &signal.input {
            char_frequency
                .entry(word.len())
                .or_default()
                .push(word.to_string());
        }

        let mut digit_mapping = HashMap::new();

        // unique known characters
        for digit in [
            SevenSegment::One,
            SevenSegment::Four,
            SevenSegment::Seven,
            SevenSegment::Eight,
        ] {
            digit_mapping.insert(digit, char_frequency[&digit.num_segments()][0].clone());
        }

        // 6 is be the only 6 segment digit that doesn't use all the segments 1 uses
        let six = signal
            .input
            .iter()
            .find(|s| {
                s.len() == SevenSegment::Six.num_segments()
                    && !digit_mapping[&SevenSegment::One]
                        .chars()
                        .all(|c| s.contains(c))
            })
            .unwrap();
        digit_mapping.insert(SevenSegment::Six, six.clone());

        // 3 is the only 5 segment digit that does use all the segments 1 uses
        let three = signal
            .input
            .iter()
            .find(|s| {
                s.len() == SevenSegment::Three.num_segments()
                    && digit_mapping[&SevenSegment::One]
                        .chars()
                        .all(|c| s.contains(c))
            })
            .unwrap();
        digit_mapping.insert(SevenSegment::Three, three.clone());

        // 9 is the only 6 segment digit that uses all the segments 3 uses
        let nine = signal
            .input
            .iter()
            .find(|s| {
                s.len() == SevenSegment::Nine.num_segments()
                    && digit_mapping[&SevenSegment::Three]
                        .chars()
                        .all(|c| s.contains(c))
            })
            .unwrap();
        digit_mapping.insert(SevenSegment::Nine, nine.clone());

        // 0 is the only remaining 6 segment digit
        let zero = signal
            .input
            .iter()
            .find(|s| {
                s.len() == SevenSegment::Zero.num_segments()
                    && s.as_str() != digit_mapping[&SevenSegment::Six]
                    && s.as_str() != digit_mapping[&SevenSegment::Nine]
            })
            .unwrap();
        digit_mapping.insert(SevenSegment::Zero, zero.clone());

        // 2 is the only 5 segment digit that shares 2 segments with 4
        let two = signal
            .input
            .iter()
            .find(|s| {
                s.len() == SevenSegment::Two.num_segments()
                    && digit_mapping[&SevenSegment::Four]
                        .chars()
                        .filter(|&c| s.contains(c))
                        .count()
                        == 2
            })
            .unwrap();
        digit_mapping.insert(SevenSegment::Two, two.clone());

        // 5 is the only remaining 5 segment digit
        let five = signal
            .input
            .iter()
            .find(|s| {
                s.len() == SevenSegment::Five.num_segments()
                    && s.as_str() != digit_mapping[&SevenSegment::Two]
                    && s.as_str() != digit_mapping[&SevenSegment::Three]
            })
            .unwrap();
        digit_mapping.insert(SevenSegment::Five, five.clone());

        let reverse_mapping: HashMap<_, _> =
            digit_mapping.into_iter().map(|(k, v)| (v, k)).collect();
        let n = signal
            .output
            .iter()
            .fold(0, |acc, o| 10 * acc + reverse_mapping[o] as u32);

        result += n;
    }

    result
}

fn main() -> Result<(), Box<dyn Error>> {
    let inputs: Vec<Signal> = input_parser::parse("puzzle8")
        .iter()
        .map(|s| Signal::from_str(s).expect("Failed to parse signal"))
        .collect();

    println!("Part 1: {}", part_1(&inputs));
    println!("Part 1: {}", part_2(&inputs));

    Ok(())
}
