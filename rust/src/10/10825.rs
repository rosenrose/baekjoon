use std::cmp::Reverse;
use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();
    let mut input = || input.next().unwrap();
    let mut output = String::new();

    let mut name_scores: Vec<_> = (0..parse_int(input()))
        .map(|_| {
            (
                input(),
                (parse_int(input()), parse_int(input()), parse_int(input())),
            )
        })
        .collect();

    name_scores.sort_unstable_by_key(|&(name, (lang, eng, math))| {
        (Reverse(lang), eng, Reverse(math), name)
    });

    for (name, _) in name_scores {
        writeln!(output, "{name}").unwrap();
    }

    print!("{output}");
}

fn parse_int(buf: &str) -> i32 {
    buf.parse().unwrap()
}
