use std::io::{stdin, Read};

fn main() {
    let stdin = stdin();
    let mut stdin = stdin.lock();

    let mut buf = String::new();
    stdin.read_to_string(&mut buf).unwrap();

    const N: i32 = 1_000_000;
    let (mut min, mut max) = (N, -N);

    buf.split_ascii_whitespace()
        .skip(1)
        .map(|s| s.parse::<i32>().unwrap())
        .for_each(|num| {
            min = num.min(min);
            max = num.max(max);
        });

    println!("{min} {max}");
}
