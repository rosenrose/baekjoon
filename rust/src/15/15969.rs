use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
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
