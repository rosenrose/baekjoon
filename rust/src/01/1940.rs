use std::collections::HashSet;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());
    input.next();

    let m = input.next().unwrap();
    let nums: HashSet<_> = input.collect();
    let count = nums.iter().filter(|&num| nums.contains(&(m - num))).count() / 2;

    println!("{count}");
}
