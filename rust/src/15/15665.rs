use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut output = String::new();

    let [_, m] = [(); 2].map(|_| input.next().unwrap());

    let mut nums: Vec<_> = input.collect();
    nums.sort();

    product(0, &mut vec![0; m], &nums, &mut output);

    print!("{output}");
}

fn product(depth: usize, selected: &mut Vec<usize>, nums: &[usize], output: &mut String) {
    if depth == selected.len() {
        for i in selected {
            write!(output, "{} ", nums[*i]).unwrap();
        }
        writeln!(output, "").unwrap();

        return;
    }

    let mut visited = [false; 10_000 + 1];

    for (i, &num) in nums.iter().enumerate() {
        if visited[num] {
            continue;
        }

        visited[num] = true;
        selected[depth] = i;

        product(depth + 1, selected, nums, output);
    }
}
