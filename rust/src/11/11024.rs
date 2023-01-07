use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    for input in buf.lines().skip(1) {
        let sum: i32 = input
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .sum();

        println!("{sum}");
    }
}
