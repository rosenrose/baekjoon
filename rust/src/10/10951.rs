use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    for line in buf.lines() {
        let sum: i32 = line
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .sum();

        println!("{sum}");
    }
}
