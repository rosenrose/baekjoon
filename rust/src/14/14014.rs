use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut output = String::new();

    let mut words: Vec<_> = buf
        .split_ascii_whitespace()
        .skip(1)
        .map(str::to_lowercase)
        .collect();

    for word in &mut words {
        if matches!(
            word.as_str(),
            "of" | "to" | "into" | "onto" | "above" | "below" | "from" | "by" | "is" | "at"
        ) {
            *word = "of".to_owned();
        }
    }

    const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

    for word in &mut words {
        let vowel_indices: Vec<_> = word.match_indices(VOWELS).map(|(i, _)| i).collect();

        *word = word
            .char_indices()
            .filter_map(|(i, ch)| {
                if !VOWELS.contains(&ch) {
                    return Some(ch);
                }

                vowel_indices[vowel_indices.len() / 2..]
                    .contains(&i)
                    .then_some(ch)
            })
            .collect();
    }

    let words: Vec<_> = words
        .iter()
        .filter_map(|word| {
            let word: String = word.matches(|ch: char| ch.is_ascii_alphabetic()).collect();
            (!word.is_empty()).then_some(word)
        })
        .collect();
    // println!("{words:?}");
    let mut line = String::new();
    const MAX_WIDTH: usize = 20;

    let mut flush_line = |line: &mut String| {
        writeln!(output, "{line}").unwrap();
        line.clear();
    };

    for word in words {
        line.extend(word.chars());
        line.push(' ');

        if line.matches(char::is_alphabetic).count() > MAX_WIDTH {
            flush_line(&mut line);
        }
    }

    if !line.is_empty() {
        flush_line(&mut line);
    }

    print!("{}", output.trim());
}
