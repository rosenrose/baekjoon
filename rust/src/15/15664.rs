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
    combinations(0, 0, &mut [0; COUNT_MAX][..m], &nums[..n], &mut output);

    print!("{output}");
}

fn combinations(
    depth: usize,
    start: usize,
    selected: &mut [usize],
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
    let mut visited = [false; NUM_MAX + 1];

    for i in start..depth + takes {
        if visited[nums[i]] {
            continue;
        }

        visited[nums[i]] = true;
        selected[depth] = i;

        combinations(depth + 1, i + 1, selected, nums, output);
    }
}
