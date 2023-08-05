use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut output = String::new();

    let [_, m] = [(); 2].map(|_| input.next().unwrap());

    let mut nums: Vec<_> = input.collect();
    nums.sort();

    permutations(0, &mut vec![0; m], &mut [false; 8], &nums, &mut output);

    print!("{output}");
}

fn permutations(
    depth: usize,
    selected: &mut Vec<usize>,
    visited_idx: &mut [bool],
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

    let mut visited_num = [false; 10_000 + 1];

    for (i, &num) in nums.iter().enumerate() {
        if visited_idx[i] || visited_num[num] {
            continue;
        }

        visited_idx[i] = true;
        visited_num[num] = true;
        selected[depth] = i;

        permutations(depth + 1, selected, visited_idx, nums, output);

        visited_idx[i] = false;
    }
}
