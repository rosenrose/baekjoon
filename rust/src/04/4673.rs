use std::collections::HashSet;
use std::fmt::Write;

fn main() {
    let mut output = String::new();
    const N: i32 = 10000;

    let d_nums: HashSet<_> = (1..N).map(func).collect();
    let self_nums = (1..=N).filter(|n| !d_nums.contains(n));

    for num in self_nums {
        writeln!(output, "{num}").unwrap();
    }

    print!("{output}");
}

fn func(n: i32) -> i32 {
    let digit_sum: i32 = n
        .to_string()
        .as_bytes()
        .iter()
        .map(|ch| (ch - b'0') as i32)
        .sum();
    n + digit_sum
}
