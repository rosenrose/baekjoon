use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let nums: Vec<_> = input.skip(1).collect();
    let diffs = (1..nums.len()).map(|i| nums[i].abs_diff(nums[i - 1]));

    println!("{}", get_gcd(diffs));
}

fn get_gcd(nums: impl Iterator<Item = u32>) -> u32 {
    nums.reduce(|mut a, mut b| loop {
        if b == 0 {
            return a;
        }

        (a, b) = (b, a % b);
    })
    .unwrap()
}
