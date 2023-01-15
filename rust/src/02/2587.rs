use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut nums: Vec<_> = buf.lines().flat_map(str::parse::<usize>).collect();
    nums.sort();

    println!("{}\n{}", nums.iter().sum::<usize>() / nums.len(), nums[2]);
}
