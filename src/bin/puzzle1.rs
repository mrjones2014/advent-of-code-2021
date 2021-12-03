use utils::input_parser;

fn main() {
    let input: Vec<i32> = input_parser::parse("puzzle1")
        .into_iter()
        .map(|line| line.parse().unwrap())
        .collect();
    let sums: Vec<i32> = input.windows(3).map(|window| window.iter().sum()).collect();
    let num_increases = sums
        .windows(2)
        .filter(|window| window[1] > window[0])
        .count();

    println!("{}", num_increases);
}
