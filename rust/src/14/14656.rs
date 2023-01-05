use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>().unwrap());

    println!(
        "{}",
        input
            .enumerate()
            .skip(1)
            .filter(|&(i, num)| i != num)
            .count()
    );
}
