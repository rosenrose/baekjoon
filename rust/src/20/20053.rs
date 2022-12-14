use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());

    const MAX: i32 = 1_000_000;

    for _ in 0..input.next().unwrap() {
        let n = input.next().unwrap();
        let (mut min, mut max) = (MAX, -MAX);

        for num in (0..n).map(|_| input.next().unwrap()) {
            min = num.min(min);
            max = num.max(max);
        }

        println!("{min} {max}");
    }
}
