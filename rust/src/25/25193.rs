use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.lines().next_back().unwrap();

    let chickens = input.chars().filter(|&ch| ch == 'C').count();
    let not_chickens = input.len() - chickens;

    println!("{}", (chickens as f64 / (not_chickens + 1) as f64).ceil());
}
