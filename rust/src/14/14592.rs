use std::cmp::Reverse;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut input = || input.next().unwrap();

    let first = (1..=input())
        .min_by_key(|_| {
            let (score, count, time) = (input(), input(), input());
            (Reverse(score), count, time)
        })
        .unwrap();

    println!("{first}");
}
