use std::fmt::Write;
use std::io;

const COUNT_MAX: usize = 8;
const NUM_MAX: usize = 10000;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut output = String::new();

    let [n, m] = [(); 2].map(|_| input.next().unwrap());
    let mut nums = [0; COUNT_MAX];

    for (i, num) in input.enumerate() {
        nums[i] = num;
    }

    nums[..n].sort();
    permutations(
        0,
        &mut [0; COUNT_MAX][..m],
        &mut [false; COUNT_MAX],
        &nums[..n],
        &mut output,
    );

    print!("{output}");
}

fn permutations(
    depth: usize,
    selected: &mut [usize],
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

    let mut visited_num = [false; NUM_MAX + 1];

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
