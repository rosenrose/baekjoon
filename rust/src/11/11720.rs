use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let sum: i32 = buf
        .lines()
        .next_back()
        .unwrap()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i32)
        .sum();

    println!("{sum}");
}
