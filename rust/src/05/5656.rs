use std::fmt::Write;
use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf.split_ascii_whitespace();
    let mut input = || input.next().unwrap();
    let mut output = String::new();

    for i in 1.. {
        let (a, op, b) = (parse_int(input()), input(), parse_int(input()));
        let cmp = match op {
            ">" => a > b,
            ">=" => a >= b,
            "<" => a < b,
            "<=" => a <= b,
            "==" => a == b,
            "!=" => a != b,
            _ => {
                break;
            }
        };

        writeln!(output, "Case {i}: {cmp}").unwrap();
    }

    print!("{output}");
}

fn parse_int(buf: &str) -> i32 {
    buf.parse().unwrap()
}
