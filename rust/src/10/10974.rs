use std::fmt::Write;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let mut output = String::new();
    let n: usize = buf.trim().parse().unwrap();

    permutations(0, &mut vec![0; n], &mut [false; 9], n, &mut output);

    print!("{output}");
}

fn permutations(
    depth: usize,
    selected: &mut Vec<usize>,
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
