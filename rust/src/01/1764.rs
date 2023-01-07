use std::collections::HashSet;
use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();
    let mut output = String::new();

    let n = parse_int(input.next().unwrap());
    input.next();

    let not_heard: HashSet<_> = (0..n).map(|_| input.next().unwrap()).collect();
    let mut not_heard_seen: Vec<_> = input
        .filter(|not_seen| not_heard.contains(not_seen))
        .collect();

    not_heard_seen.sort();

    writeln!(output, "{}", not_heard_seen.len()).unwrap();
    writeln!(output, "{}", not_heard_seen.join("\n")).unwrap();

    print!("{output}");
}

fn parse_int(buf: &str) -> i32 {
    buf.parse().unwrap()
}
