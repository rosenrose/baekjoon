use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut output = String::new();

    let (_, m) = (input.next(), input.next().unwrap());

    let mut nums: Vec<_> = input.collect();
    nums.sort();

    permutations(0, &mut vec![0; m], &nums, &mut output);

    print!("{output}");
}

fn permutations(depth: usize, selected: &mut Vec<usize>, nums: &Vec<usize>, output: &mut String) {
    if depth == selected.len() {
        for num in selected {
            write!(output, "{num} ").unwrap();
        }
        writeln!(output, "").unwrap();

        return;
    }

    for &num in nums {
        if selected[..depth].contains(&num) {
            continue;
        }

        selected[depth] = num;
        permutations(depth + 1, selected, nums, output);
    }
}
