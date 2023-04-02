use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();
    let mut input = || input.next().unwrap();
    let mut output = String::new();

    let mut members: Vec<_> = (0..parse_int(input()))
        .map(|order| {
            let (age, name) = (parse_int(input()), input());
            (age, order, name)
        })
        .collect();

    members.sort_unstable();

    for (age, _, name) in members {
        writeln!(output, "{age} {name}").unwrap();
    }

    print!("{output}");
}

fn parse_int(buf: &str) -> i32 {
    buf.parse().unwrap()
}
