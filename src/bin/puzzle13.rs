use std::{collections::HashSet, error::Error, fmt::Display, str::FromStr};
use utils::input_parser;

const GRID_WIDTH: u32 = 40;
const GRID_HEIGHT: u32 = 6;

type Position = (u32, u32);

#[derive(Clone)]
enum Fold {
    X(u32),
    Y(u32),
}

impl FromStr for Fold {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let relevant_part = &s[("fold along ".len())..];
        let (direction, value) = relevant_part
            .split_once('=')
            .expect("No delimeter '=' found in fold input");
        Ok(match direction {
            "x" => Fold::X(value.parse().expect("Failed to parse u32 from fold value")),
            "y" => Fold::Y(value.parse().expect("Failed to parse u32 from fold value")),
            _ => panic!("Invalid fold direction '{}'", direction),
        })
    }
}

struct Sheet {
    pub dots: HashSet<Position>,
}

impl Sheet {
    pub fn fold(&mut self, fold: &Fold) {
        match fold {
            Fold::X(fold) => {
                let dots: Vec<_> = self
                    .dots
                    .iter()
                    .copied()
                    .filter(|&(x, _)| x > *fold)
                    .collect();
                for pos in &dots {
                    self.dots.remove(pos);
                }

                for (x, y) in dots {
                    self.dots.insert((2 * fold - x, y));
                }
            }
            Fold::Y(fold) => {
                let dots: Vec<_> = self
                    .dots
                    .iter()
                    .copied()
                    .filter(|&(_, y)| y > *fold)
                    .collect();
                for pos in &dots {
                    self.dots.remove(pos);
                }

                for (x, y) in dots {
                    self.dots.insert((x, 2 * fold - y));
                }
            }
        }
    }
}

impl Display for Sheet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut self_str = String::from("");
        (0..GRID_HEIGHT).for_each(|y| {
            (0..GRID_WIDTH).for_each(|x| {
                self_str += format!(
                    "{}",
                    if self.dots.contains(&(x, y)) {
                        '#'
                    } else {
                        ' '
                    }
                )
                .as_str();
            });
            self_str += "\n";
        });
        writeln!(f, "{}", self_str)
    }
}

fn part_1(sheet: &mut Sheet, fold: &Fold) {
    sheet.fold(fold);
}

fn part_2(sheet: &mut Sheet, folds: &[Fold]) {
    folds.iter().for_each(|fold| sheet.fold(fold))
}

fn main() -> Result<(), Box<dyn Error>> {
    let all_input = input_parser::parse("puzzle13");
    let divider = all_input
        .iter()
        .position(|line| line.is_empty())
        .expect("No divider line found.");
    let dots: HashSet<Position> = all_input[0..divider]
        .iter()
        .map(|input| {
            let (x_str, y_str) = input.split_once(',').expect("No delimeter found in input");
            (
                x_str.parse().expect("Failed to parse u32 from x input"),
                y_str.parse().expect("Failed to parse u32 from y input"),
            )
        })
        .collect();
    let folds: Vec<Fold> = all_input[(divider + 1)..]
        .iter()
        .map(|input| Fold::from_str(input).expect("Failed to parse fold"))
        .collect();

    let mut part_1_sheet = Sheet { dots: dots.clone() };
    part_1(&mut part_1_sheet, &folds[0]);
    println!("Part 1: {}", part_1_sheet.dots.len());

    let mut part_2_sheet = Sheet { dots };
    part_2(&mut part_2_sheet, &folds);
    println!("Part 2:\n{}", part_2_sheet);

    Ok(())
}
