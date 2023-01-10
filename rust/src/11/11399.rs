use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());

    let mut times: Vec<_> = input.skip(1).collect();
    times.sort();

    let waits = times.iter().fold(vec![0], |mut acc, time| {
        acc.push(acc.last().unwrap() + time);
        acc
    });

    println!("{}", waits.iter().sum::<i32>());
}
