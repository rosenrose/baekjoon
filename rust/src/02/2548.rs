use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());

    let mut nums: Vec<_> = input.skip(1).collect();
    nums.sort();

    let len = nums.len();

    if len % 2 == 1 {
        println!("{}", nums[len / 2]);
        return;
    }

    let (a, b) = (nums[len / 2 - 1], nums[len / 2]);
    let diff_a = nums.iter().map(|num| num.abs_diff(a)).sum::<u32>();
    let diff_b = nums.iter().map(|num| num.abs_diff(b)).sum::<u32>();

    println!("{}", if diff_a <= diff_b { a } else { b });
}
