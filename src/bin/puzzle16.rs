use std::error::Error;
use utils::input_parser;

fn numeric_value(c: &[char]) -> usize {
    let c: String = c.iter().collect();
    usize::from_str_radix(&c, 2).unwrap()
}

enum Operator {
    Sum(usize),
    Product(usize),
    Min(usize),
    Max(usize),
    Gt(Vec<usize>),
    Lt(Vec<usize>),
    Eq(Vec<usize>),
}

impl Operator {
    fn insert(&mut self, val: usize) {
        match self {
            Operator::Sum(state) => {
                *state += val;
            }
            Operator::Product(state) => {
                *state *= val;
            }
            Operator::Min(state) => {
                *state = (*state).min(val);
            }
            Operator::Max(state) => {
                *state = (*state).max(val);
            }
            Operator::Gt(vals) | Operator::Lt(vals) | Operator::Eq(vals) => {
                vals.push(val);
            }
        }
    }

    fn val(&self) -> usize {
        match self {
            Operator::Sum(val)
            | Operator::Product(val)
            | Operator::Min(val)
            | Operator::Max(val) => *val,
            Operator::Gt(vals) => {
                if vals[0] > vals[1] {
                    1
                } else {
                    0
                }
            }
            Operator::Lt(vals) => {
                if vals[0] < vals[1] {
                    1
                } else {
                    0
                }
            }
            Operator::Eq(vals) => {
                if vals[0] == vals[1] {
                    1
                } else {
                    0
                }
            }
        }
    }
}

fn parse(i: &mut usize, decoded: &[char], part_1: bool) -> usize {
    let version = numeric_value(&decoded[*i..*i + 3]);
    let type_id = numeric_value(&decoded[*i + 3..*i + 6]);
    *i += 6;

    if type_id == 4 {
        // Literal
        let mut continues = true;

        let mut val = Vec::new();
        while continues {
            continues = decoded[*i] == '1';
            let mut decoded: Vec<char> = decoded[*i + 1..*i + 5].iter().copied().collect();
            val.append(&mut decoded);

            *i += 5;
        }
        let val = numeric_value(&val);
        return if part_1 { version } else { val };
    }

    let mut op = if part_1 {
        Operator::Sum(version)
    } else {
        match type_id {
            0 => Operator::Sum(0),
            1 => Operator::Product(1),
            2 => Operator::Min(usize::MAX),
            3 => Operator::Max(0),
            5 => Operator::Gt(vec![]),
            6 => Operator::Lt(vec![]),
            7 => Operator::Eq(vec![]),
            _ => panic!(),
        }
    };

    let length_id = decoded[*i];
    *i += 1;
    if length_id == '0' {
        let len = numeric_value(&decoded[*i..*i + 15]);
        *i += 15;
        let stop_at = *i + len;
        while *i < stop_at {
            op.insert(parse(i, decoded, part_1));
        }
        assert!(*i == stop_at);
    } else if length_id == '1' {
        let len = numeric_value(&decoded[*i..*i + 11]);
        *i += 11;
        for _ in 0..len {
            op.insert(parse(i, decoded, part_1));
        }
    }

    op.val()
}

pub fn solve(input: String, part_1: bool) -> usize {
    let mut decoded = String::new();
    for c in input.trim().chars() {
        decoded += &format!("{:04b}", c.to_digit(16).unwrap());
    }
    let mut i = 0;
    let decoded: Vec<char> = decoded.chars().collect();
    let mut version_sum = 0;
    while i < decoded.len() {
        if decoded.len() - i < 20 && numeric_value(&decoded[i..]) == 0 {
            break;
        }
        version_sum += parse(&mut i, &decoded, part_1);
    }
    version_sum
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = input_parser::parse("puzzle16")[0].clone();
    println!("Part 1: {}", solve(input.clone(), true));
    println!("Part 2: {}", solve(input, false));
    Ok(())
}
