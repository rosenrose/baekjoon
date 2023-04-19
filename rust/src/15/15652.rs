use std::fmt::Write;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let mut output = String::new();
    let [n, m] = parse_int_vec(&buf)[..] else { return };

    combinations_with_replacement(0, 0, &mut vec![0; m], n, &mut output);

    print!("{output}");
}

fn combinations_with_replacement(
    depth: usize,
    start: usize,
    selected: &mut Vec<usize>,
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

fn parse_int_vec(buf: &String) -> Vec<usize> {
    buf.split_whitespace().flat_map(str::parse).collect()
}
