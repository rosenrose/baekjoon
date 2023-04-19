use std::fmt::Write;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let mut output = String::new();
    let [n, m] = parse_int_vec(&buf)[..] else { return };

    permutations(0, &mut vec![0; m], n, &mut output);

    print!("{output}");
}

fn permutations(depth: usize, selected: &mut Vec<usize>, nums: usize, output: &mut String) {
    if depth == selected.len() {
        for num in selected {
            write!(output, "{num} ").unwrap();
        }
        writeln!(output, "").unwrap();

        return;
    }

    for num in 1..=nums {
        if selected[..depth].contains(&num) {
            continue;
        }

        selected[depth] = num;
        permutations(depth + 1, selected, nums, output);
    }
}

fn parse_int_vec(buf: &String) -> Vec<usize> {
    buf.split_whitespace().flat_map(str::parse).collect()
}
