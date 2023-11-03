use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.lines().next_back().unwrap();

    let chickens = input.matches('C').count();
    let not_chickens = input.len() - chickens;

    println!("{}", chickens.div_ceil(not_chickens + 1));
}
