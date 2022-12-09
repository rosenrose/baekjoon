use std::cmp::Ordering;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf.split_ascii_whitespace();
    let mut output = String::new();

    for _ in 0..parse_int(input.next().unwrap()) {
        let games = parse_int(input.next().unwrap());

        let (a, b) = (0..games)
            .map(|_| match (input.next().unwrap(), input.next().unwrap()) {
                ("R", "P") => (0, 1),
                ("R", "S") => (1, 0),
                ("P", "R") => (1, 0),
                ("P", "S") => (0, 1),
                ("S", "R") => (0, 1),
                ("S", "P") => (1, 0),
                _ => (0, 0),
            })
            .reduce(|(a1, b1), (a2, b2)| (a1 + a2, b1 + b2))
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
