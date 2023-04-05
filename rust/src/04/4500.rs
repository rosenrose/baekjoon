use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines();
    let mut input = || input.next().unwrap();
    let mut output = String::new();

    for _ in 0..parse_int(input()) {
        let mut names: Vec<_> = input().split(' ').collect();
        let p = names.len();
        let (chooser, n) = (input(), parse_int(input()));

        let chooser_idx = names.iter().position(|&name| name == chooser).unwrap();
        names.rotate_left(chooser_idx);
        // println!("{names:?}")
        let winner_idx = if n % p == 0 { p } else { n % p } - 1;
        writeln!(output, "{}", names[winner_idx]).unwrap();
    }

    print!("{output}");
}

fn parse_int(buf: &str) -> usize {
    buf.parse().unwrap()
}
