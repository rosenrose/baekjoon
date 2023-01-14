use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());

    let (n, k) = (input.next().unwrap(), input.next().unwrap());
    let mut nums = Vec::with_capacity(n as usize);

    for num in input {
        nums.push(num);
    }
    nums.sort_unstable();

    println!("{}", nums[k as usize - 1]);
}
