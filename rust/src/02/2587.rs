use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut nums: Vec<_> = buf.lines().map(|s| s.parse::<usize>().unwrap()).collect();
    nums.sort();

    println!("{}\n{}", nums.iter().sum::<usize>() / nums.len(), nums[2]);
}
