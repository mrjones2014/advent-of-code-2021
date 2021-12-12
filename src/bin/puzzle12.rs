use std::{
    collections::{HashMap, HashSet},
    error::Error,
};
use utils::input_parser;

type Map = HashMap<String, Vec<String>>;

fn parse_input() -> Map {
    let input = input_parser::parse("puzzle12");
    let mut edges = Map::new();
    for line in input.iter() {
        let (from, to) = line.split_once('-').expect("Failed to parse graph");
        edges
            .entry(from.to_owned())
            .or_default()
            .push(to.to_owned());
        edges
            .entry(to.to_owned())
            .or_default()
            .push(from.to_owned());
    }

    edges
}

fn is_small(cave: String) -> bool {
    cave.chars().next().unwrap().is_lowercase()
}

fn spelunk(
    map: &Map,
    mut path: Vec<String>,
    from: String,
    visited: HashSet<String>,
    allow_revisit_small_cave: bool,
) -> usize {
    path.push(from.clone());
    if from == "end" {
        return 1;
    }

    let mut count = 0;
    for to in map.get(&from).expect("Expected a node") {
        let mut visited = visited.clone();
        let mut allow_revisit_small_cave = allow_revisit_small_cave;
        if is_small(to.to_string()) {
            if visited.contains(to) {
                if !allow_revisit_small_cave || *to == "start" || *to == "end" {
                    continue;
                }

                allow_revisit_small_cave = false;
            } else {
                visited.insert(to.to_string());
            }
        }

        count += spelunk(
            map,
            path.clone(),
            to.to_string(),
            visited,
            allow_revisit_small_cave,
        );
    }
    count
}

fn part_1(map: Map) -> usize {
    let visited = HashSet::from([String::from("start")]);
    spelunk(&map, Vec::new(), String::from("start"), visited, false)
}

fn part_2(map: Map) -> usize {
    let visited = HashSet::from([String::from("start")]);
    spelunk(&map, Vec::new(), String::from("start"), visited, true)
}

fn main() -> Result<(), Box<dyn Error>> {
    let map = parse_input();
    println!("Part 1: {}", part_1(map.clone()));
    println!("Part 2: {}", part_2(map));
    Ok(())
}
