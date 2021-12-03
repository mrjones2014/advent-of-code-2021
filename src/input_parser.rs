use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn parse(puzzle_num: &str) -> Vec<String> {
    let file_path = format!("./inputs/{}", puzzle_num);
    let file = File::open(file_path).unwrap();
    BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .collect()
}
