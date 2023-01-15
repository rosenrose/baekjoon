use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut output = String::new();

    while let Some(k @ 7..) = input.next() {
        let nums: Vec<_> = (0..k).map(|_| input.next().unwrap()).collect();

        combination(&nums, 6, 0, &mut Vec::new(), &mut output);

        writeln!(output, "").unwrap();
    }

    print!("{output}")
}

fn combination(
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

    for i in start..nums.len() - (m as usize - 1) {
        if selected.contains(&nums[i]) {
            continue;
        }

        selected.push(nums[i]);

        combination(nums, m - 1, i + 1, selected, output);

        selected.pop();
    }
}
