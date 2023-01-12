use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());
    let mut input = || input.next().unwrap();

    const MAX: i32 = 1_000_000;

    for _ in 0..input() {
        let (min, max) = (0..input()).fold((MAX, -MAX), |(min, max), _| {
            let num = input();
            (num.min(min), num.max(max))
        });

        println!("{min} {max}");
    }
}
