use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    const MAX: i32 = 1_000_000;
    let (mut min, mut max) = (MAX, -MAX);

    buf.split_ascii_whitespace()
        .skip(1)
        .map(|s| s.parse::<i32>().unwrap())
        .for_each(|num| {
            min = num.min(min);
            max = num.max(max);
        });

    println!("{min} {max}");
}
