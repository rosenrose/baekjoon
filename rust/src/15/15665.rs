use std::collections::HashSet;
use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());
    let mut output = String::new();

    let (_, m) = (input.next(), input.next().unwrap());

    let mut nums: Vec<_> = input.collect();
    nums.sort();

    product(&nums, m, &mut Vec::new(), &mut output);

    print!("{output}");
}

fn product(nums: &Vec<i32>, m: i32, selected: &mut Vec<usize>, output: &mut String) {
    if m == 0 {
        for &i in selected.iter() {
            write!(output, "{} ", nums[i]).unwrap();
        }
        writeln!(output, "").unwrap();

        return;
    }

    let mut visited = HashSet::new();

    for (i, &num) in nums.iter().enumerate() {
        if visited.contains(&num) {
            continue;
        }

        selected.push(i);
        visited.insert(num);

        product(nums, m - 1, selected, output);

        selected.pop();
    }
}
