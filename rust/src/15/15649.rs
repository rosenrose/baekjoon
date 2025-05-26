use std::fmt::Write;
use std::io;

const MAX: usize = 8;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut output = String::new();

    let [n, m] = [(); 2].map(|_| input.next().unwrap());

    permutations(0, &mut [0; MAX][..m], &mut [false; 9], n, &mut output);

    print!("{output}");
}

fn permutations(
    depth: usize,
    selected: &mut [usize],
    visited: &mut [bool],
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

    for num in 1..=nums {
        if visited[num] {
            continue;
        }

        visited[num] = true;
        selected[depth] = num;

        permutations(depth + 1, selected, visited, nums, output);

        visited[num] = false;
    }
}
