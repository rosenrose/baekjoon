use std::collections::HashSet;
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
    let mut result = HashSet::new();

    combination_with_replacement(&nums, m, 0, &mut selected, &mut result);

    let mut result: Vec<_> = result.iter().collect();
    result.sort_unstable();

    for arr in result {
        for num in arr {
            write!(output, "{num} ").unwrap()
        }
        writeln!(output, "").unwrap();
    }

    print!("{output}");
}

fn combination_with_replacement(
    nums: &Vec<i32>,
    m: i32,
    start: usize,
    selected: &mut Vec<usize>,
    result: &mut HashSet<Vec<i32>>,
) {
    if m == 0 {
        result.insert(selected.iter().map(|&i| nums[i]).collect());

        return;
    }

    for i in start..nums.len() {
        selected.push(i);

        combination_with_replacement(nums, m - 1, i, selected, result);

        selected.pop();
    }
}
