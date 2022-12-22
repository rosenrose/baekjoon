use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());

    let mut max = -1000;

    let _ = input.skip(1).fold(0, |acc, num| {
        let sum = num.max(acc + num);
        max = sum.max(max);

        sum
    });

    println!("{max}");
}
