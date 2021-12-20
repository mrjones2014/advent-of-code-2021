use std::error::Error;
use utils::input_parser;

#[derive(Debug, Clone)]
struct Tree {
    values: Vec<u32>,
    depths: Vec<u32>,
}

impl Tree {
    fn parse(s: &str) -> Tree {
        let mut t = Tree {
            values: Vec::new(),
            depths: Vec::new(),
        };

        let mut depth = 0;
        for c in s.chars() {
            match c {
                '[' => {
                    depth += 1;
                }
                ',' => (),
                ']' => {
                    depth -= 1;
                }
                d => {
                    t.values.push(d.to_digit(10).unwrap());
                    t.depths.push(depth - 1);
                }
            }
        }

        t
    }

    fn explodes(&mut self) -> bool {
        for i in 0..self.depths.len() {
            let depth = self.depths[i];
            if depth != 4 {
                continue;
            }

            // add left value to left neighbor
            if i != 0 {
                self.values[i - 1] += self.values[i];
            }

            // add right value to right neighbor
            if i + 2 < self.values.len() {
                self.values[i + 2] += self.values[i + 1];
            }

            self.values[i] = 0;
            self.depths[i] = 3;
            self.values.remove(i + 1);
            self.depths.remove(i + 1);

            return true;
        }

        false
    }

    fn splits(&mut self) -> bool {
        for i in 0..self.values.len() {
            let v = self.values[i];
            if v < 10 {
                continue;
            }

            let (a, b) = if v % 2 == 0 {
                (v / 2, v / 2)
            } else {
                (v / 2, v / 2 + 1)
            };

            self.values[i] = a;
            self.depths[i] += 1;
            self.values.insert(i + 1, b);
            self.depths.insert(i + 1, self.depths[i]);

            return true;
        }

        false
    }

    fn reduce(&mut self) {
        loop {
            if !self.explodes() && !self.splits() {
                break;
            }
        }
    }

    fn merge(&mut self, other: &Tree) {
        self.values.extend(other.values.iter());
        self.depths.extend(other.depths.iter());
        for i in 0..self.depths.len() {
            self.depths[i] += 1;
        }
    }

    fn score(&self) -> u32 {
        let mut vals = self.values.clone();
        let mut depths = self.depths.clone();

        while vals.len() > 1 {
            for i in 0..depths.len() - 1 {
                if depths[i] == depths[i + 1] {
                    vals[i] = 3 * vals[i] + 2 * vals[i + 1];
                    vals.remove(i + 1);
                    depths.remove(i + 1);

                    if depths[i] > 0 {
                        depths[i] -= 1;
                    }

                    break;
                }
            }
        }

        vals[0]
    }
}

fn part_1(input: &[String]) -> u32 {
    let mut iter = input.iter();
    let mut tree = Tree::parse(iter.next().unwrap());
    for line in iter {
        tree.merge(&Tree::parse(line));
        tree.reduce();
    }

    tree.score()
}

fn part_2(input: &[String]) -> u32 {
    let mut best_score = 0;
    let trees: Vec<Tree> = input.iter().map(|line| Tree::parse(line)).collect();
    for a in trees.iter() {
        for b in trees.iter() {
            let mut a = a.clone();
            a.merge(&b);
            a.reduce();
            best_score = best_score.max(a.score());
        }
    }

    best_score
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = input_parser::parse("puzzle18");

    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));

    Ok(())
}
