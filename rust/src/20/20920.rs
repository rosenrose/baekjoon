use std::cmp::Reverse;
use std::collections::HashMap;
use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();
    let mut output = String::new();

    let (n, m) = (
        parse_int(input.next().unwrap()),
        parse_int(input.next().unwrap()),
    );

    let mut word_counts = HashMap::with_capacity(n);

    for word in input.filter(|word| word.len() >= m) {
        word_counts.entry(word).and_modify(|c| *c += 1).or_insert(1);
    }

    let mut word_counts = Vec::from_iter(word_counts);
    word_counts.sort_unstable_by_key(|&(word, count)| (Reverse(count), Reverse(word.len()), word));

    for (word, _) in word_counts {
        writeln!(output, "{word}").unwrap();
    }

    print!("{output}");
}

fn parse_int(buf: &str) -> usize {
    buf.parse().unwrap()
}
