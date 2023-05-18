use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();
    let mut input = || input.next().unwrap();
    let mut output = String::new();

    let n: i32 = input().parse().unwrap();
    let get_counts = |s: &str| {
        let mut counts = [0; 26];

        for ch in s.as_bytes() {
            counts[(ch - b'a') as usize] += 1;
        }

        counts
    };

    for (a, b) in (0..n).map(|_| (input(), input())) {
        writeln!(
            output,
            "{}",
            if get_counts(a) == get_counts(b) {
                "Possible"
            } else {
                "Impossible"
            }
        )
        .unwrap();
    }

    print!("{output}");
}
