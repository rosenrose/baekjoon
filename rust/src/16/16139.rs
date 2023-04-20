use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();
    let mut input = || input.next().unwrap();
    let mut output = String::new();

    let s = input();
    let q = parse_int(input());
    let mut sum_accum = vec![[0; 26]; s.len() + 1];

    for (row, ch) in s.char_indices() {
        let idx = ch as usize - 'a' as usize;

        for col in 0..26 {
            sum_accum[row + 1][col] = sum_accum[row][col] + if col == idx { 1 } else { 0 };
        }
    }

    for (ch, left, right) in (0..q).map(|_| (input(), parse_int(input()), parse_int(input()))) {
        let idx = ch.as_bytes()[0] as usize - 'a' as usize;

        writeln!(
            output,
            "{}",
            sum_accum[right + 1][idx] - sum_accum[left][idx]
        )
        .unwrap();
    }

    print!("{output}");
}

fn parse_int(buf: &str) -> usize {
    buf.parse().unwrap()
}
