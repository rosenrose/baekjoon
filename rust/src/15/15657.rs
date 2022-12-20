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

    combination_with_replacement(&nums, m, 0, &mut selected, &mut output);

    print!("{output}");
}

fn combination_with_replacement(
    nums: &Vec<i32>,
    m: i32,
    start: usize,
    selected: &mut Vec<i32>,
    output: &mut String,
) {
    if m == 0 {
        for num in selected {
            write!(output, "{num} ").unwrap();
        }
        writeln!(output, "").unwrap();

        return;
    }

    for i in start..nums.len() {
        selected.push(nums[i]);

        combination_with_replacement(nums, m - 1, i, selected, output);

        selected.pop();
    }
}
