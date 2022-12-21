use std::fmt::Write;
use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());
    let mut output = String::new();

    input.next();
    let m = input.next().unwrap();

    let mut nums: Vec<_> = input.collect();
    nums.sort();

    let mut selected = Vec::new();
    let mut result = Vec::new();

    product(&nums, m, 0, &mut selected, &mut result);

    result.sort_unstable();
    result.dedup();

    for arr in result {
        for num in arr {
            write!(output, "{num} ").unwrap()
        }
        writeln!(output, "").unwrap();
    }

    print!("{output}");
}

fn product(
    nums: &Vec<i32>,
    m: i32,
    start: usize,
    selected: &mut Vec<usize>,
    result: &mut Vec<Vec<i32>>,
) {
    if m == 0 {
        result.push(selected.iter().map(|&i| nums[i]).collect());

        return;
    }

    for (i, _) in nums.iter().enumerate().skip(start) {
        selected.push(i);

        product(nums, m - 1, 0, selected, result);

        selected.pop();
    }
}
