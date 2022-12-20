use std::fmt::Write;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let mut output = String::new();

    if let [n, m] = parse_int_vec(&buf)[..] {
        let nums: Vec<_> = (1..=n).collect();
        let mut selected = Vec::new();

        permutation(&nums, m, 0, &mut selected, &mut output);

        print!("{output}");
    }
}

fn permutation(
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

    for &num in nums.iter().skip(start) {
        if selected.contains(&num) {
            continue;
        }

        selected.push(num);

        permutation(nums, m - 1, 0, selected, output);

        selected.pop();
    }
}

fn parse_int_vec(buf: &String) -> Vec<i32> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}
