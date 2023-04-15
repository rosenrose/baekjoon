use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();
    let mut input = || input.next().unwrap();
    let mut output = String::new();

    let s = input();
    let q = parse_int(input());
    let mut sum_accum = vec![vec![0; s.len() + 1]; 26];

    for (i, ch) in s.char_indices() {
        let idx = ch as usize - 'a' as usize;

        for row in 0..sum_accum.len() {
            sum_accum[row][i + 1] = sum_accum[row][i] + if row == idx { 1 } else { 0 };
        }
    }

    for (ch, left, right) in (0..q).map(|_| (input(), parse_int(input()), parse_int(input()))) {
        let idx = ch.as_bytes()[0] as usize - 'a' as usize;

        writeln!(
            output,
            "{}",
            sum_accum[idx][right + 1] - sum_accum[idx][left]
        )
        .unwrap();
    }

    print!("{output}");
}

fn parse_int(buf: &str) -> usize {
    buf.parse().unwrap()
}
