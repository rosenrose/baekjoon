use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines();
    let mut input = || input.next();
    let mut output = String::new();

    let n: i32 = input().unwrap().parse().unwrap();
    let langs: Vec<(_, Vec<_>)> = (0..n)
        .map(|_| {
            let mut tokens = input().unwrap().split_whitespace();
            let lang = tokens.next().unwrap();

            (lang, tokens.map(str::to_lowercase).collect())
        })
        .collect();
    input();
    // println!("{langs:?}");
    while let Some(sample) = input() {
        let sample_words: Vec<_> = sample
            .split([',', '.', '!', ';', '?', '(', ')', ' '])
            .map(str::to_lowercase)
            .collect();
        // println!("{sample_words:?}");
        let (lang, _) = langs
            .iter()
            .find(|(_, keywords)| {
                keywords
                    .iter()
                    .any(|keyword| sample_words.contains(keyword))
            })
            .unwrap();

        writeln!(output, "{lang}").unwrap();
    }

    print!("{output}");
}
