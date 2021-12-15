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

fn part_1(input: Vec<Vec<u32>>, height: usize, width: usize) -> u32 {
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

fn main() -> Result<(), Box<dyn Error>> {
    let input: Vec<Vec<u32>> = input_parser::parse("puzzle15")
        .iter()
        .map(|line| {
            line.chars()
                .map(|value| value.to_digit(10).expect("Failed to parse digit"))
                .collect()
        })
        .collect();
    let height = input.len();
    let width = input[0].len();
    println!("Part 1: {}", part_1(input, height, width));
    Ok(())
}
