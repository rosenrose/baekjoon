use std::collections::HashMap;
use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines();
    let mut output = String::new();

    let n: usize = input.next().unwrap().parse().unwrap();
    let mut dict = HashMap::with_capacity(n);

    for word in input.by_ref().take(n) {
        let key = get_side_chars_and_counts(word);
        dict.entry(key).and_modify(|c| *c += 1).or_insert(1);
    }
    // println!("{dict:?}");
    for sentence in input.skip(1) {
        let count: i32 = sentence
            .split(' ')
            .filter_map(|word| {
                (!word.is_empty()).then(|| {
                    let key = get_side_chars_and_counts(word);
                    dict.get(&key).unwrap_or(&0)
                })
            })
            .product();

        writeln!(output, "{count}").unwrap();
    }

    print!("{output}");
}

fn get_side_chars_and_counts(word: &str) -> ((u8, u8), [i32; 52]) {
    let mut counts = [0; 52];
    let chars = word.as_bytes();
    let len = chars.len();

    let first = chars[0];

    if len == 1 {
        return ((first, 0), counts);
    }

    let last = chars[len - 1];

    for ch in &chars[1..len - 1] {
        let idx = (ch
            - if matches!(ch, b'a'..=b'z') {
                b'a'
            } else {
                b'A' + 26
            }) as usize;

        counts[idx] += 1;
    }

    ((first, last), counts)
}
