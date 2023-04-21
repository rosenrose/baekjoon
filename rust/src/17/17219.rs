use std::collections::HashMap;
use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();
    let mut input = || input.next().unwrap();
    let mut output = String::new();

    let (n, m) = (parse_int(input()), parse_int(input()));
    let passwords: HashMap<_, _> = (0..n).map(|_| (input(), input())).collect();

    for site in (0..m).map(|_| input()) {
        writeln!(output, "{}", passwords[site]).unwrap();
    }

    print!("{output}");
}

fn parse_int(buf: &str) -> i32 {
    buf.parse().unwrap()
}
