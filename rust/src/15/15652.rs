use std::fmt::Write;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let mut output = String::new();

    if let [n, m] = parse_int_vec(&buf)[..] {
        let nums: Vec<_> = (1..=n).collect();
        let mut selected = Vec::new();

        combination_with_replacement(&nums, m, 0, &mut selected, &mut output);

        print!("{output}");
    }
}

fn combination_with_replacement(
    nums: &Vec<i32>,
    m: i32,
    start: usize,
    selected: &mut Vec<i32>,
    output: &mut String,
) {
    if m == 0 {
        for num in selected {
            write!(output, "{num} ").unwrap();
        }
        writeln!(output, "").unwrap();

        return;
    }

    for i in start..nums.len() {
        selected.push(nums[i]);

        combination_with_replacement(nums, m - 1, i, selected, output);

        selected.pop();
    }
}

fn parse_int_vec(buf: &String) -> Vec<i32> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}