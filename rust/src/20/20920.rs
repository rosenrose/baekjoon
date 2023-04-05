use std::collections::HashMap;
use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();
    let mut output = String::new();

    let (_, m) = (
        input.next(),
        input.next().unwrap().parse::<usize>().unwrap(),
    );

    let word_counts = input
        .filter(|word| word.len() >= m)
        .fold(HashMap::new(), |mut acc, word| {
            acc.entry(word).and_modify(|c| *c += 1).or_insert(1);
            acc
        });

    let mut word_counts = Vec::from_iter(word_counts);
    word_counts.sort_unstable_by(|(a_word, a_count), (b_word, b_count)| {
        b_count
            .cmp(a_count)
            .then_with(|| b_word.len().cmp(&a_word.len()))
            .then_with(|| a_word.cmp(b_word))
    });

    for (word, _) in word_counts {
        writeln!(output, "{word}").unwrap();
    }

    print!("{output}");
}
