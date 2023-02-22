use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut nums: Vec<_> = buf.lines().flat_map(str::parse::<usize>).collect();

    let avg = nums.iter().sum::<usize>() / nums.len();
    let mid = nums.select_nth_unstable(2).1;

    println!("{avg}\n{mid}");
}
