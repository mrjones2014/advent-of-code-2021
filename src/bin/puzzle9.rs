use std::{
    collections::{HashMap, HashSet},
    error::Error,
    ops::Index,
};
use utils::input_parser;

const MAX_HEIGHT: u32 = 9;

struct Map {
    heights: HashMap<(i32, i32), u32>,
    width: usize,
    height: usize,
}

impl From<Vec<String>> for Map {
    fn from(input: Vec<String>) -> Self {
        let mut heights = HashMap::new();
        let mut height = 0;
        let mut width = 0;
        for (y, line) in input.iter().enumerate() {
            for (x, value) in line.chars().enumerate() {
                heights.insert(
                    (x as i32, y as i32),
                    value.to_digit(10).expect("Failed to parse value"),
                );
            }

            height += 1;
            width = line.len();
        }

        Self {
            heights,
            height,
            width,
        }
    }
}

impl Index<(i32, i32)> for Map {
    type Output = u32;

    fn index(&self, index: (i32, i32)) -> &Self::Output {
        &self.heights[&index]
    }
}

impl Map {
    pub fn contains(&self, pos: (i32, i32)) -> bool {
        self.heights.contains_key(&pos)
    }

    pub fn get(&self, pos: (i32, i32)) -> Option<u32> {
        self.heights.get(&pos).copied()
    }

    fn adjacent(&self, pos: (i32, i32)) -> Vec<(i32, i32)> {
        let mut adjacent = Vec::new();
        for (delta_x, delta_y) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
            let adjacent_value = (pos.0 + delta_x, pos.1 + delta_y);
            if self.contains(adjacent_value) {
                adjacent.push(adjacent_value);
            }
        }

        adjacent
    }
}

fn part_1(map: &Map) -> u32 {
    let mut result = 0;

    for y in 0..map.height {
        for x in 0..map.width {
            let pos = (x as i32, y as i32);
            if let Some(height) = map.get(pos) {
                if map.adjacent(pos).into_iter().all(|p| map[p] > height) {
                    result += 1 + height;
                }
            }
        }
    }

    result
}

fn part_2(map: &Map) -> u32 {
    let mut basins = Vec::new();
    let mut seen = HashSet::new();

    for y in 0..map.height {
        for x in 0..map.width {
            let pos = (x as i32, y as i32);
            if map.contains(pos) && map[pos] < MAX_HEIGHT && !seen.contains(&pos) {
                // New basin
                seen.insert(pos);
                let mut size = 1;

                // Depth first search
                let mut edge = vec![pos];
                while let Some(pos) = edge.pop() {
                    let adjacent: Vec<_> = map
                        .adjacent(pos)
                        .into_iter()
                        .filter(|&p| !seen.contains(&p) && map[p] < MAX_HEIGHT)
                        .collect();
                    for adj in adjacent {
                        seen.insert(adj);
                        size += 1;

                        edge.push(adj);
                    }
                }

                basins.push(size);
            }
        }
    }

    basins.sort_unstable();

    // Multiply three largest basins
    basins[basins.len() - 3..].iter().product()
}

fn main() -> Result<(), Box<dyn Error>> {
    let inputs: Vec<String> = input_parser::parse("puzzle9");
    let map = Map::from(inputs);
    println!("Part 1: {}", part_1(&map));
    println!("Part 2: {}", part_2(&map));
    Ok(())
}
