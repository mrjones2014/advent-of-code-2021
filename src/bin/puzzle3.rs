use std::error::Error;

use utils::input_parser;

const NUM_BITS: usize = 12;

fn main() -> Result<(), Box<dyn Error>> {
    let input = input_parser::parse("puzzle3");
    let mut num_binary_per_col = [(0, 0); NUM_BITS]; // for each column, (num 0s, num 1s)
    for line in input.iter() {
        for (column, bit) in line.chars().enumerate() {
            match bit {
                '0' => {
                    num_binary_per_col[column] = (
                        num_binary_per_col[column].0 + 1,
                        num_binary_per_col[column].1,
                    )
                }
                '1' => {
                    num_binary_per_col[column] = (
                        num_binary_per_col[column].0,
                        num_binary_per_col[column].1 + 1,
                    )
                }
                _ => panic!("{} is not valid binary", bit),
            }
        }
    }
    let gamma_rate_binary_str = num_binary_per_col
        .into_iter()
        .map(|(num_zeroes, num_ones)| if num_zeroes > num_ones { "0" } else { "1" })
        .collect::<Vec<&str>>()
        .join("");
    let gamma_rate = i64::from_str_radix(&gamma_rate_binary_str, 2)?;
    let epsilon_rate_binary_str = num_binary_per_col
        .into_iter()
        .map(|(num_zeroes, num_ones)| if num_zeroes > num_ones { "1" } else { "0" })
        .collect::<Vec<&str>>()
        .join("");
    let epsilon_rate = i64::from_str_radix(&epsilon_rate_binary_str, 2)?;
    println!("Power consumption: {}", gamma_rate * epsilon_rate);
    Ok(())
}
