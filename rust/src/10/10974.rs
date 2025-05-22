use std::fmt::Write;
use std::io;

const MAX: usize = 8;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut output = String::new();

    let n: usize = buf.trim().parse().unwrap();
    permutations(0, &mut [0; MAX][..n], &mut [false; MAX + 1], &mut output);

    print!("{output}");
}

fn permutations(depth: usize, selected: &mut [usize], visited: &mut [bool], output: &mut String) {
    if depth == selected.len() {
        for num in selected {
            write!(output, "{num} ").unwrap();
        }
        writeln!(output, "").unwrap();

        return;
    }

    for num in 1..=selected.len() {
        if visited[num] {
            continue;
        }

        visited[num] = true;
        selected[depth] = num;

        permutations(depth + 1, selected, visited, output);

        visited[num] = false;
    }
}
