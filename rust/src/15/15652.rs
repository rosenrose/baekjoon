use std::fmt::Write;
use std::io;

const MAX: usize = 8;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut output = String::new();

    let [n, m] = [(); 2].map(|_| input.next().unwrap());

    combinations_with_replacement(0, 0, &mut [0; MAX][..m], n, &mut output);

    print!("{output}");
}

fn combinations_with_replacement(
    depth: usize,
    start: usize,
    selected: &mut [usize],
    nums: usize,
    output: &mut String,
) {
    if depth == selected.len() {
        for num in selected {
            write!(output, "{num} ").unwrap();
        }
        writeln!(output, "").unwrap();

        return;
    }

    for (i, num) in (1..=nums).enumerate().skip(start) {
        selected[depth] = num;
        combinations_with_replacement(depth + 1, i, selected, nums, output);
    }
}
