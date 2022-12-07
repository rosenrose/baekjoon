use std::collections::HashSet;
use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

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
