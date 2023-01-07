use std::cmp::Ordering;
use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();
    let mut input = || input.next().unwrap();
    let mut output = String::new();

    for _ in 0..parse_int(input()) {
        let games = parse_int(input());

        let (a, b) = (0..games)
            .map(|_| match (input(), input()) {
                ("R", "P") | ("P", "S") | ("S", "R") => (0, 1),
                ("R", "S") | ("P", "R") | ("S", "P") => (1, 0),
                _ => (0, 0),
            })
            .reduce(|a, b| (a.0 + b.0, a.1 + b.1))
            .unwrap();

        let result = match a.cmp(&b) {
            Ordering::Greater => "Player 1",
            Ordering::Equal => "TIE",
            Ordering::Less => "Player 2",
        };

        writeln!(output, "{result}").unwrap();
    }

    print!("{output}");
}

fn parse_int(buf: &str) -> i32 {
    buf.parse().unwrap()
}
