use std::collections::HashSet;
use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.split_ascii_whitespace();
    let mut output = String::new();

    let words: HashSet<_> = input.skip(1).collect();
    let mut words = Vec::from_iter(words);

    words.sort_by(|a, b| {
        if a.len() == b.len() {
            a.cmp(b)
        } else {
            a.len().cmp(&b.len())
        }
    });

    for word in words {
        writeln!(output, "{word}").unwrap();
    }

    print!("{output}");
}
