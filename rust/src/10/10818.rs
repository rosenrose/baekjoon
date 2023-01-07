use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
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
