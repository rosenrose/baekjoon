use std::collections::HashSet;
use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());

    let mut nums = Vec::new();
    let mut nums_set = HashSet::new();

    for num in input.skip(1) {
        nums.push(num);
        nums_set.insert(num);
    }

    let mut min_diff_sum = u32::MAX;

    let diff_sums: Vec<(i32, u32)> = nums_set
        .iter()
        .map(|&num| {
            let diff_sum = nums.iter().map(|n| n.abs_diff(num)).sum::<u32>();
            min_diff_sum = diff_sum.min(min_diff_sum);

            (num, diff_sum)
        })
        .collect();

    let represents = diff_sums
        .iter()
        .filter_map(|&(num, diff)| (diff == min_diff_sum).then(|| num));

    let represent = represents.min().unwrap();

    println!("{represent}");
}
