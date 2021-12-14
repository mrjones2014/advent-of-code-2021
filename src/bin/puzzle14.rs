use std::{collections::HashMap, error::Error};
use utils::input_parser;

pub fn count(
    polymer1: char,
    polymer2: char,
    reactions: &HashMap<(char, char), char>,
    memo: &mut HashMap<(char, char, usize), HashMap<char, usize>>,
    iterations: usize,
) -> HashMap<char, usize> {
    if iterations == 0 {
        let mut counts = HashMap::new();
        *counts.entry(polymer1).or_default() += 1;
        *counts.entry(polymer2).or_default() += 1;
        return counts;
    }

    if let Some(result) = memo.get(&(polymer1, polymer2, iterations)) {
        return result.clone();
    }

    let compound_option = reactions.get(&(polymer1, polymer2));
    if compound_option.is_none() {
        let mut counts = HashMap::new();
        *counts.entry(polymer1).or_default() += 1;
        *counts.entry(polymer2).or_default() += 1;
        return counts;
    }

    let compound = compound_option.unwrap();
    let mut counts = HashMap::new();
    count(polymer1, *compound, reactions, memo, iterations - 1)
        .into_iter()
        .for_each(|(c, count)| *counts.entry(c).or_default() += count);
    count(*compound, polymer2, reactions, memo, iterations - 1)
        .into_iter()
        .for_each(|(c, count)| *counts.entry(c).or_default() += count);
    *counts.entry(*compound).or_default() -= 1;
    memo.insert((polymer1, polymer2, iterations), counts.clone());
    counts
}

fn solve(input: &[String], num_iterations: usize) -> usize {
    let template: Vec<char> = input[0].chars().collect();
    let mut reactions = HashMap::new();

    for line in input.iter().skip(2) {
        let (pattern, addition) = line.split_once(" -> ").unwrap();
        let mut pattern = pattern.chars();
        let p1 = pattern.next().unwrap();
        let p2 = pattern.next().unwrap();
        let addition = addition.chars().next().unwrap();
        reactions.insert((p1, p2), addition);
    }

    let mut memo: HashMap<(char, char, usize), HashMap<char, usize>> = HashMap::new();
    let mut counts: HashMap<char, usize> = HashMap::new();

    for i in 1..template.len() {
        let p1 = template[i - 1];
        let p2 = template[i];
        for (c, count) in count(p1, p2, &reactions, &mut memo, num_iterations) {
            *counts.entry(c).or_default() += count;
        }

        if i != 1 {
            *counts.entry(p1).or_default() -= 1;
        }
    }

    let most_common = counts.values().max().unwrap();
    let least_common = counts.values().min().unwrap();
    most_common - least_common
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = input_parser::parse("puzzle14");
    println!("Part 1: {}", solve(&input, 10));
    println!("Part 1: {}", solve(&input, 40));
    Ok(())
}
