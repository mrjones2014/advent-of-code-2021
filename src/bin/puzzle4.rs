use std::{collections::HashMap, error::Error, fmt::Display};

use utils::input_parser;

const BOARD_DIMENSIONS: usize = 5;

// 2D array, 5x5 matrix
type BoardRows = [[u32; BOARD_DIMENSIONS]; BOARD_DIMENSIONS];

struct Board {
    input_str: String, // so we can print it for debugging
    rows: BoardRows,
}

impl Board {
    pub fn is_win(&self, seen_input: &[u32]) -> bool {
        // check rows
        if self.rows.iter().any(|row| {
            row.iter()
                .all(|cell| seen_input.iter().any(|input| cell == input))
        }) {
            return true;
        }

        // check columns
        for i in 0..BOARD_DIMENSIONS {
            if self
                .rows
                .iter()
                .all(|row| seen_input.iter().any(|input| row[i] == *input))
            {
                return true;
            }
        }

        false
    }

    pub fn compute_score(&self, seen_input: &[u32]) -> u32 {
        let all_cells_iter = self.rows.iter().flatten();
        let mut sum = 0;
        for value in all_cells_iter {
            if seen_input.iter().all(|input| input != value) {
                sum += value;
            }
        }
        sum * seen_input.last().expect("Failed to get last input")
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.input_str)
    }
}

impl From<&[String; BOARD_DIMENSIONS]> for Board {
    fn from(rows_input: &[String; BOARD_DIMENSIONS]) -> Self {
        let mut rows = Vec::new();
        for row in rows_input.iter() {
            let cells_in_row: [u32; BOARD_DIMENSIONS] = row
                .trim()
                .split(' ')
                .filter(|cell| !(*cell).is_empty() && *cell != " ")
                .map(|cell_str| cell_str.parse().expect("Unable to parse cell"))
                .collect::<Vec<u32>>()
                .try_into()
                .expect("Unable to parse cell");
            rows.push(cells_in_row);
        }

        Board {
            rows: rows.try_into().expect("Unable to parse rows"),
            input_str: rows_input.join("\n"),
        }
    }
}

fn parse_boards(input_data: Vec<String>) -> Vec<Board> {
    let boards_strs = input_data
        .iter()
        .skip(2)
        .filter(|row| *row != "\n" && !row.is_empty())
        .map(|row| row.to_string())
        .collect::<Vec<String>>();
    let boards_input: Vec<&[String; BOARD_DIMENSIONS]> = boards_strs
        .chunks(BOARD_DIMENSIONS)
        .map(|row| row.try_into().expect("Incorrect row length"))
        .collect::<Vec<&[String; BOARD_DIMENSIONS]>>();
    boards_input
        .iter()
        .map(|rows| Board::from(*rows))
        .to_owned()
        .collect()
}

fn find_first_winning_board<'a>(boards: &'a [Board], inputs: &'a [u32]) -> (&'a Board, &'a [u32]) {
    for i in 0..inputs.len() {
        for board in boards.iter() {
            let seen_inputs = &inputs[0..i];
            if board.is_win(seen_inputs) {
                return (board, seen_inputs);
            }
        }
    }

    panic!("No winning boards")
}

fn find_last_winning_board<'a>(boards: &'a [Board], inputs: &'a [u32]) -> (&'a Board, &'a [u32]) {
    let mut turns_to_win: HashMap<usize, (&Board, &[u32])> = HashMap::new();
    for board in boards.iter() {
        for i in 0..inputs.len() {
            let seen_inputs = &inputs[0..i];
            if board.is_win(seen_inputs) {
                turns_to_win.insert(i, (board, seen_inputs));
                break;
            }
        }
    }

    *turns_to_win
        .iter()
        .max_by_key(|entry| entry.0)
        .expect("No winning boards")
        .1
}

fn main() -> Result<(), Box<dyn Error>> {
    let input_data = input_parser::parse("puzzle4");
    let bingo_inputs = &input_data[0]
        .split(',')
        .map(|num_str| num_str.parse::<u32>().expect("Unable to parse input value"))
        .collect::<Vec<u32>>();
    let boards = parse_boards(input_data);
    let (first_winning_board, first_winning_used_inputs) =
        find_first_winning_board(&boards, bingo_inputs);
    let (last_winning_board, last_winning_used_inputs) =
        find_last_winning_board(&boards, bingo_inputs);
    println!(
        "Score of first winning board: {}",
        first_winning_board.compute_score(first_winning_used_inputs)
    );

    println!(
        "Score of last winning board: {}",
        last_winning_board.compute_score(last_winning_used_inputs)
    );

    Ok(())
}
