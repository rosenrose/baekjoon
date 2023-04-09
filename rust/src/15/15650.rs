use std::fmt::Write;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let mut output = String::new();
    let [n, m] = parse_int_vec(&buf)[..] else { return };

    let nums: Vec<_> = (1..=n).collect();

    combinations(0, 0, &mut vec![0; m], &nums, &mut output);

    print!("{output}");
}

fn combinations(
    depth: usize,
    start: usize,
    selected: &mut Vec<usize>,
    nums: &Vec<usize>,
    output: &mut String,
) {
    if depth == selected.len() {
        for num in selected {
            write!(output, "{num} ").unwrap();
        }
        writeln!(output, "").unwrap();

        return;
    }

    let takes = nums.len() - selected.len() + 1;

    for (i, &num) in nums.iter().enumerate().skip(start).take(takes) {
        if selected[..depth].contains(&num) {
            continue;
        }

        selected[depth] = num;
        combinations(depth + 1, i + 1, selected, nums, output);
    }
}

fn parse_int_vec(buf: &String) -> Vec<usize> {
    buf.split_whitespace().flat_map(str::parse).collect()
}
