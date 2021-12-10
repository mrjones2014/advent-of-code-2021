use std::{char, error::Error};
use utils::input_parser;

struct Stack<T> {
    values: Vec<T>,
}

impl<T> Stack<T> {
    pub fn push(&mut self, value: T) {
        self.values.push(value);
    }

    pub fn pop(&mut self) -> Option<T> {
        match self.values.is_empty() {
            true => None,
            false => self.values.pop(),
        }
    }

    pub fn has_next(&self) -> bool {
        !self.values.is_empty()
    }

    pub fn new() -> Self {
        Self { values: Vec::new() }
    }
}

fn syntax_points(syntax: char) -> u64 {
    match syntax {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!("Unknown syntax: {}", syntax),
    }
}

fn completion_points(syntax: char) -> u64 {
    match syntax {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => panic!("Unknown syntax: {}", syntax),
    }
}

fn closing(syntax: char) -> char {
    match syntax {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => panic!("Unknown syntax: {}", syntax),
    }
}

fn part_1(input: &[String]) -> u64 {
    let mut score: u64 = 0;
    input.iter().for_each(|line| {
        let mut syntax_stack = Stack::<char>::new();
        for syntax in line.chars() {
            match syntax {
                '<' | '{' | '(' | '[' => syntax_stack.push(syntax),
                '>' | '}' | ')' | ']' => {
                    if syntax_stack.has_next() && closing(syntax_stack.pop().unwrap()) != syntax {
                        score += syntax_points(syntax);
                        break;
                    }
                }
                _ => panic!("Unknown syntax: {}", syntax),
            }
        }
    });

    score
}

fn part_2(input: &[String]) -> u64 {
    let mut scores = Vec::new();

    input.iter().for_each(|line| {
        let mut syntax_stack = Stack::<char>::new();
        let mut line_is_valid = true;
        for syntax in line.chars() {
            match syntax {
                '<' | '{' | '(' | '[' => syntax_stack.push(syntax),
                '>' | '}' | ')' | ']' => {
                    if syntax_stack.has_next() && closing(syntax_stack.pop().unwrap()) != syntax {
                        line_is_valid = false;
                        break;
                    }
                }
                _ => panic!("Unknown syntax: {}", syntax),
            }
        }

        if line_is_valid {
            let mut score = 0;
            while syntax_stack.has_next() {
                score = score * 5 + completion_points(closing(syntax_stack.pop().unwrap()));
            }

            scores.push(score);
        }
    });

    scores.sort_unstable();

    scores[scores.len() / 2]
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = input_parser::parse("puzzle10");
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
    Ok(())
}
