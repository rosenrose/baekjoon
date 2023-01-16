use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let nums: Vec<_> = input.skip(1).collect();
    let score = nums[0]
        + (1..nums.len())
            .filter_map(|i| (nums[i] - nums[i - 1] > 1).then_some(nums[i]))
            .sum::<i32>();

    println!("{score}");
}
