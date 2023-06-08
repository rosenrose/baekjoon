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

    let takes = nums.len() - (selected.len() - 1);
    let mut visited = [false; 10_000 + 1];

    for i in start..depth + takes {
        if visited[nums[i]] {
            continue;
        }

        visited[nums[i]] = true;
        selected[depth] = i;

        combinations(depth + 1, i + 1, selected, nums, output);
    }
}
