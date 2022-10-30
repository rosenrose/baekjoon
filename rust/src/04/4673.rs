use std::collections::HashSet;
use std::io::{stdout, BufWriter, Write};

fn main() {
    let stdout = stdout();
    let mut stdout = BufWriter::new(stdout.lock());

    const N: i32 = 10000;

    let d_nums: HashSet<i32> = (1..N).map(|n| d(n)).collect();
    let self_nums = (1..=N).filter(|n| !d_nums.contains(n));

    for num in self_nums {
        writeln!(stdout, "{num}").unwrap();
    }
}

fn d(n: i32) -> i32 {
    n + n
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i32)
        .sum::<i32>()
}
