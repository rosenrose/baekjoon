use std::collections::HashSet;
use std::fmt::Write;

fn main() {
    let mut output = String::new();
    const N: i32 = 10000;

    let d_nums: HashSet<_> = (1..N).map(|n| d(n)).collect();
    let self_nums = (1..=N).filter(|n| !d_nums.contains(n));

    for num in self_nums {
        writeln!(output, "{num}").unwrap();
    }

    print!("{output}");
}

fn d(n: i32) -> i32 {
    n + n
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i32)
        .sum::<i32>()
}
