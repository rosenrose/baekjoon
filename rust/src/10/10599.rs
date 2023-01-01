use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());

    while let (Some(a), Some(b), Some(c), Some(d)) =
        (input.next(), input.next(), input.next(), input.next())
    {
        if (a, b, c, d) == (0, 0, 0, 0) {
            return;
        }

        let ages = [c - a, c - b, d - a, d - b];

        println!(
            "{} {}",
            ages.iter().min().unwrap(),
            ages.iter().max().unwrap()
        );
    }
}
