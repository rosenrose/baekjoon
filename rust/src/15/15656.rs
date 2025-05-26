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
    product(0, &mut [0; MAX][..m], &nums[..n], &mut output);

    print!("{output}");
}

fn product(depth: usize, selected: &mut [usize], nums: &[usize], output: &mut String) {
    if depth == selected.len() {
        for num in selected {
            write!(output, "{num} ").unwrap();
        }
        writeln!(output, "").unwrap();

        return;
    }

    for &num in nums {
        selected[depth] = num;
        product(depth + 1, selected, nums, output);
    }
}
