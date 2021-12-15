use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
    error::Error,
};
use utils::input_parser;

type Position = (usize, usize);

#[derive(Clone, Copy, PartialEq, Eq)]
struct Node {
    position: Position,
    cost: u32,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn adjacencies(position: (usize, usize), height: usize, width: usize) -> Vec<(usize, usize)> {
    let mut adjacencies = Vec::new();

    // up
    if position.0 > 0 {
        adjacencies.push((position.0 - 1, position.1));
    }

    // down
    if position.0 < height - 1 {
        adjacencies.push((position.0 + 1, position.1));
    }

    // left
    if position.1 > 0 {
        adjacencies.push((position.0, position.1 - 1));
    }

    // right
    if position.1 < width - 1 {
        adjacencies.push((position.0, position.1 + 1));
    }

    adjacencies
}

fn part_1(input: &[Vec<u32>]) -> u32 {
    let height = input.len();
    let width = input[0].len();
    let mut path = BinaryHeap::new();
    path.push(Node {
        position: (0, 1),
        cost: input[0][1],
    });
    path.push(Node {
        position: (1, 0),
        cost: input[1][0],
    });
    let mut path_costs = HashMap::new();
    path_costs.insert((0, 0), 0);

    while let Some(node) = path.pop() {
        // if we're at the bottom right corner, we found the destination
        if node.position == (height - 1, width - 1) {
            return node.cost;
        }

        if let Some(cost) = path_costs.get(&node.position) {
            if *cost <= node.cost {
                continue;
            }
        }

        for adjacent in adjacencies(node.position, height, width) {
            let cost = node.cost + input[adjacent.0][adjacent.1];
            if let Some(cost_for_adjacent) = path_costs.get(&adjacent) {
                if *cost_for_adjacent <= cost {
                    continue;
                }
            }

            path.push(Node {
                position: adjacent,
                cost,
            });

            path_costs.insert(node.position, node.cost);
        }
    }

    unreachable!()
}

fn build_part_2_map(input: &[Vec<u32>]) -> Vec<Vec<u32>> {
    let tile_height = input.len();
    let tile_width = input[0].len();

    let mut new_map = vec![vec![0; 5 * tile_width]; 5 * tile_height];

    for tile_row in 0..5 {
        for tile_column in 0..5 {
            for i in 0..tile_height {
                for j in 0..tile_width {
                    let mut new_risk_value = input[i][j] + tile_row + tile_column;
                    // values > 9 wrap back to 1
                    if new_risk_value > 9 {
                        new_risk_value -= 9;
                    }
                    new_map[tile_row as usize * tile_height + i]
                        [tile_column as usize * tile_width + j] = new_risk_value;
                }
            }
        }
    }

    new_map
}

fn part_2(input: &[Vec<u32>]) -> u32 {
    let new_map = build_part_2_map(input);
    part_1(&new_map)
}

fn main() -> Result<(), Box<dyn Error>> {
    let input: Vec<Vec<u32>> = input_parser::parse("puzzle15")
        .iter()
        .map(|line| {
            line.chars()
                .map(|value| value.to_digit(10).expect("Failed to parse digit"))
                .collect()
        })
        .collect();
    println!("Part 1: {}", part_1(&input));
    println!("Part 1: {}", part_2(&input));
    Ok(())
}
