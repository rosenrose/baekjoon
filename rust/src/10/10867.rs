use std::collections::HashSet;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let nums: HashSet<_> = buf
        .split_ascii_whitespace()
        .skip(1)
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    let mut nums: Vec<_> = nums.iter().collect();
    nums.sort_unstable();

    for num in nums {
        print!("{num} ");
    }
}
