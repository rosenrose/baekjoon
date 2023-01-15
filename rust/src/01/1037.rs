use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let mut nums: Vec<_> = input.skip(1).collect();
    nums.sort();

    println!("{}", nums[0] * nums.last().unwrap());
}
