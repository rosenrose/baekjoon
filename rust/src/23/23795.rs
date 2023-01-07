use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let sum: i32 = buf
        .lines()
        .rev()
        .skip(1)
        .map(|s| s.parse::<i32>().unwrap())
        .sum();

    println!("{sum}");
}
