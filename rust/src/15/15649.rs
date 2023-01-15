use std::fmt::Write;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let mut output = String::new();
    let [n, m] = parse_int_vec(&buf)[..] else { return };

    let nums: Vec<_> = (1..=n).collect();

    permutation(&nums, m, &mut Vec::new(), &mut output);

    print!("{output}");
}

fn permutation(nums: &Vec<i32>, m: i32, selected: &mut Vec<i32>, output: &mut String) {
    if m == 0 {
        for num in selected {
            write!(output, "{num} ").unwrap();
        }
        writeln!(output, "").unwrap();

        return;
    }

    for &num in nums {
        if selected.contains(&num) {
            continue;
        }

        selected.push(num);

        permutation(nums, m - 1, selected, output);

        selected.pop();
    }
}

fn parse_int_vec(buf: &String) -> Vec<i32> {
    buf.split_whitespace().flat_map(str::parse).collect()
}
