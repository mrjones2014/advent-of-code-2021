use std::{
    error::Error,
    ops::{Add, Div, Range, RangeInclusive},
};
use utils::input_parser;

struct Target {
    pub x_range: (i64, i64),
    pub y_range: (i64, i64),
}

impl Target {
    pub fn max_y_if_hits(&self, velocity_x: i64, velocity_y: i64) -> Option<i64> {
        let (mut dx, mut dy) = (velocity_x, velocity_y);
        let (mut peak, mut x, mut y) = (0, 0, 0);
        loop {
            x += dx;
            y += dy;

            dx += if dx > 0 {
                -1
            } else if dx < 0 {
                1
            } else {
                0
            };
            dy -= 1;

            peak = peak.max(y);

            // if we've hit the target box
            if x >= self.x_range.0
                && x <= self.x_range.1
                && y >= self.y_range.0
                && y <= self.y_range.1
            {
                return Some(peak);
            }

            if x > self.x_range.1 && y > self.y_range.1 {
                return None;
            }

            if dx == 0 && !(x >= self.x_range.0 && x <= self.x_range.1) {
                return None;
            }

            if dy < 0 && y < self.y_range.0 {
                return None;
            }
        }
    }
}

fn compute_velocity_bounds(target: &Target) -> (RangeInclusive<i64>, RangeInclusive<i64>) {
    let x_lower_bound = 0;
    let x_upper_bound = target.x_range.1.pow(2).add(target.x_range.1).div(2);
    let y_lower_bound = target.y_range.0;
    let y_upper_bound = ((target.y_range.0 * -1) - 1).abs();
    (x_lower_bound..=x_upper_bound, y_lower_bound..=y_upper_bound)
}

fn solve(target: &Target) -> (i64, i64) {
    let mut num_solutions = 0;
    let mut max_y_optimal_path = 0;
    let (x_bounds, y_bounds) = compute_velocity_bounds(target);
    println!("{:?}, {:?}", y_bounds, x_bounds);
    x_bounds.for_each(|dx| {
        y_bounds.clone().for_each(|dy| {
            if let Some(max_y) = target.max_y_if_hits(dx, dy) {
                max_y_optimal_path = max_y_optimal_path.max(max_y);
                num_solutions += 1;
            }
        })
    });
    (max_y_optimal_path, num_solutions)
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = input_parser::parse("puzzle17")[0].clone();
    let (x_str, y_str) = input
        .trim()
        .trim_start_matches("target area: ")
        .split_once(", ")
        .expect("No delimeter found");
    let (x1_str, x2_str) = x_str
        .trim_start_matches("x=")
        .split_once("..")
        .expect("Input is in invalid format");
    let (y1_str, y2_str) = y_str
        .trim_start_matches("y=")
        .split_once("..")
        .expect("Input is in invalid format");
    let x1: i64 = x1_str.parse().expect("Failed to parse i64");
    let x2: i64 = x2_str.parse().expect("Failed to parse i64");
    let y1: i64 = y1_str.parse().expect("Failed to parse i64");
    let y2: i64 = y2_str.parse().expect("Failed to parse i64");
    let target = Target {
        x_range: (x1.min(x2), x1.max(x2)),
        y_range: (y1.min(y2), y1.max(y2)),
    };
    let (part_1, part_2) = solve(&target);
    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);
    Ok(())
}
