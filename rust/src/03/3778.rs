use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines();
    let mut input = || input.next().unwrap();
    let mut output = String::new();

    let n: i32 = input().parse().unwrap();
    let get_counts = |s: &str| {
        let mut counts = [0_i32; 26];

        for ch in s.as_bytes() {
            counts[(ch - b'a') as usize] += 1;
        }

        counts
    };

    for (i, (a, b)) in (1..=n).map(|i| (i, (input(), input()))) {
        let a_counts = get_counts(a);
        let b_counts = get_counts(b);
        let dist: u32 = a_counts
            .iter()
            .zip(b_counts)
            .map(|(a, b)| a.abs_diff(b))
            .sum();

        writeln!(output, "Case #{i}: {dist}").unwrap();
    }

    print!("{output}");
}
