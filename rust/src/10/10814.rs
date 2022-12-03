use std::fmt::Write;
use std::io::{stdin, Read};

fn main() {
    let stdin = stdin();
    let mut stdin = stdin.lock();

    let mut buf = String::new();
    stdin.read_to_string(&mut buf).unwrap();

    let mut input = buf.split_ascii_whitespace();
    let mut output = String::new();

    let mut members: Vec<_> = (0..parse_int(input.next().unwrap()))
        .map(|order| {
            let age = parse_int(input.next().unwrap());
            let name = input.next().unwrap();

            (age, name, order)
        })
        .collect();

    members.sort_by(|(age1, _, order1), (age2, _, order2)| {
        if age1 == age2 {
            order1.cmp(order2)
        } else {
            age1.cmp(age2)
        }
    });

    for (age, name, _) in members {
        writeln!(output, "{age} {name}").unwrap();
    }

    print!("{output}");
}

fn parse_int(buf: &str) -> i32 {
    buf.parse().unwrap()
}
