use utils::input_parser;

fn main() {
    let input = input_parser::parse("puzzle1");
    println!("{}", input.join("\n"))
}
