use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut output = String::new();

    let (_, m) = (input.next(), input.next().unwrap());

    let mut nums: Vec<_> = input.collect();
    nums.sort();

    permutations(0, &mut vec![0; m], &mut [false; 8], &nums, &mut output);

    print!("{output}");
}

fn permutations(
    depth: usize,
    selected: &mut Vec<usize>,
    visited: &mut [bool],
    nums: &[usize],
    output: &mut String,
) {
    if depth == selected.len() {
        for i in selected {
            write!(output, "{} ", nums[*i]).unwrap();
        }
        writeln!(output, "").unwrap();

        return;
    }

    for i in 0..nums.len() {
        if visited[i] {
            continue;
        }

        visited[i] = true;
        selected[depth] = i;

        permutations(depth + 1, selected, visited, nums, output);

        visited[i] = false;
    }
}
