use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());

    let (mut min, mut max) = (1000, 0);

    for score in input.skip(1) {
        min = score.min(min);
        max = score.max(max);
    }

    println!("{}", max - min);
}
