use std::collections::HashSet;
use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut output = String::new();

    let (_, m) = (input.next(), input.next().unwrap());

    let mut nums: Vec<_> = input.collect();
    nums.sort();

    combinations(0, 0, &mut vec![0; m], &nums, &mut output);

    print!("{output}");
}

fn combinations(
    depth: usize,
    start: usize,
    selected: &mut Vec<usize>,
    nums: &Vec<usize>,
    output: &mut String,
) {
    if depth == selected.len() {
        for &i in selected.iter() {
            write!(output, "{} ", nums[i]).unwrap();
        }
        writeln!(output, "").unwrap();

        return;
    }

    let takes = nums.len() - selected.len() + 1;
    let mut visited = HashSet::new();

    for (i, &num) in nums.iter().enumerate().skip(start).take(takes) {
        if visited.contains(&num) {
            continue;
        }

        visited.insert(num);
        selected[depth] = i;

        combinations(depth + 1, i + 1, selected, nums, output);
    }
}
