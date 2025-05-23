use std::fmt::Write;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let mut output = String::new();
    let [n, m] = parse_int_vec(&buf)[..] else {
        return;
    };

    product(0, &mut vec![0; m], n, &mut output);

    print!("{output}");
}

fn product(depth: usize, selected: &mut Vec<usize>, nums: usize, output: &mut String) {
    if depth == selected.len() {
        for num in selected {
            write!(output, "{num} ").unwrap();
        }
        writeln!(output, "").unwrap();

        return;
    }

    for num in 1..=nums {
        selected[depth] = num;
        product(depth + 1, selected, nums, output);
    }
}

fn parse_int_vec(buf: &str) -> Vec<usize> {
    buf.split_whitespace().flat_map(str::parse).collect()
}
