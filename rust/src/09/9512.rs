use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines();
    let mut output = String::new();

    let n: i32 = input.next().unwrap().parse().unwrap();
    let langs: Vec<(_, Vec<_>)> = (0..n)
        .map(|_| {
            let mut it = input.next().unwrap().split(' ');
            let lang = it.next().unwrap();

            (lang, it.map(str::to_lowercase).collect())
        })
        .collect();
    // println!("{langs:?}");
    for sample in input.skip(1) {
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
