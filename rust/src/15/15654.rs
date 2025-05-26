use std::fmt::Write;
use std::io;

const MAX: usize = 8;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut output = String::new();

    let [n, m] = [(); 2].map(|_| input.next().unwrap());
    let mut nums = [0; MAX];

    for (i, num) in input.enumerate() {
        nums[i] = num;
    }

    nums[..n].sort();
    permutations(
        0,
        &mut [0; MAX][..m],
        &mut [false; MAX],
        &nums[..n],
        &mut output,
    );

    print!("{output}");
}

fn permutations(
    depth: usize,
    selected: &mut [usize],
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
