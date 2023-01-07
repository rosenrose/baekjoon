use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut nums: Vec<_> = buf.lines().map(|s| s.parse::<usize>().unwrap()).collect();
    nums.sort();

    println!("{}\n{}", nums.iter().sum::<usize>() / nums.len(), nums[2]);
}
