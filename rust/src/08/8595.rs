use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let sum: i64 = buf
        .lines()
        .next_back()
        .unwrap()
        .split(char::is_alphabetic)
        .flat_map(str::parse::<i64>)
        .sum();

    println!("{sum}");
}
