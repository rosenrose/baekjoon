use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());

    let nums: Vec<_> = input.skip(1).collect();
    let diffs = (1..nums.len()).map(|i| nums[i].abs_diff(nums[i - 1]));

    println!("{}", get_gcd(diffs));
}

fn get_gcd<I>(nums: I) -> u32
where
    I: Iterator<Item = u32>,
{
    nums.reduce(|mut a, mut b| loop {
        if b == 0 {
            return a;
        }

        (a, b) = (b, a % b);
    })
    .unwrap()
}
