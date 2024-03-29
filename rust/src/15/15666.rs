use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut output = String::new();

    let [_, m] = [(); 2].map(|_| input.next().unwrap());

    let mut nums: Vec<_> = input.collect();
    nums.sort();

    combinations_with_replacement(0, 0, &mut vec![0; m], &nums, &mut output);

    print!("{output}");
}

fn combinations_with_replacement(
    depth: usize,
    start: usize,
    selected: &mut Vec<usize>,
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

    let mut visited = [false; 10_000 + 1];

    for i in start..nums.len() {
        if visited[nums[i]] {
            continue;
        }

        visited[nums[i]] = true;
        selected[depth] = i;

        combinations_with_replacement(depth + 1, i, selected, nums, output);
    }
}
