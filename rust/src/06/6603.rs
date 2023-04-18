use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut output = String::new();

    while let Some(k @ 7..) = input.next() {
        let nums: Vec<_> = input.by_ref().take(k).collect();

        combinations(0, 0, &mut [0; 6], &nums, &mut output);

        writeln!(output, "").unwrap();
    }

    print!("{output}")
}

fn combinations(
    depth: usize,
    start: usize,
    selected: &mut [usize; 6],
    nums: &Vec<usize>,
    output: &mut String,
) {
    if depth == selected.len() {
        for num in selected {
            write!(output, "{num} ").unwrap();
        }
        writeln!(output, "").unwrap();

        return;
    }

    let takes = nums.len() - selected.len() + 1;

    for (i, &num) in nums.iter().enumerate().skip(start).take(takes) {
        selected[depth] = num;
        combinations(depth + 1, i + 1, selected, nums, output);
    }
}
