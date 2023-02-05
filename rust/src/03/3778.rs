use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines();
    let mut input = || input.next().unwrap();
    let mut output = String::new();

    let get_count = |s: &str| {
        s.chars().fold([0_i32; 26], |mut acc, ch| {
            acc[ch as usize - 'a' as usize] += 1;
            acc
        })
    };

    for (i, (a, b)) in (1..=parse_int(input())).map(|i| (i, (input(), input()))) {
        let a_counts = get_count(a);
        let b_counts = get_count(b);
        let dist: u32 = a_counts
            .iter()
            .zip(b_counts)
            .map(|(a, b)| a.abs_diff(b))
            .sum();

        writeln!(output, "Case #{i}: {dist}").unwrap();
    }

    print!("{output}");
}

fn parse_int(buf: &str) -> i32 {
    buf.parse().unwrap()
}
