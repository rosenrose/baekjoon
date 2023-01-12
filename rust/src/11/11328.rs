use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();
    let mut input = || input.next().unwrap();
    let mut output = String::new();

    let n: i32 = input().parse().unwrap();
    let char_count = |s: &str| {
        let mut count = [0; 26];

        for ch in s.chars() {
            count[ch as usize - 'a' as usize] += 1;
        }

        count
    };

    for (a, b) in (0..n).map(|_| (input(), input())) {
        writeln!(
            output,
            "{}",
            if char_count(a) == char_count(b) {
                "Possible"
            } else {
                "Impossible"
            }
        )
        .unwrap();
    }

    print!("{output}");
}
