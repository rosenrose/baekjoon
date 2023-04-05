use std::collections::HashMap;
use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines();
    let mut output = String::new();

    let n: usize = input.next().unwrap().parse().unwrap();
    let mut dict = HashMap::new();

    for word in input.by_ref().take(n) {
        let key = get_side_chars_and_count(word);
        dict.entry(key).and_modify(|c| *c += 1).or_insert(1);
    }
    // println!("{dict:?}");
    for sentence in input.skip(1) {
        let count: i32 = sentence
            .split(' ')
            .filter_map(|word| {
                (!word.is_empty()).then(|| {
                    let key = get_side_chars_and_count(word);
                    dict.get(&key).unwrap_or(&0)
                })
            })
            .product();

        writeln!(output, "{count}").unwrap();
    }

    print!("{output}");
}

fn get_side_chars_and_count(word: &str) -> ((char, char), [i32; 52]) {
    let mut counts = [0; 52];
    let mut chars = word.chars();

    let first = chars.next().unwrap();
    let Some(last) = chars.next_back() else {
        return ((first, '\0'), counts);
    };

    for ch in chars {
        let idx = ch as usize
            - if matches!(ch, 'a'..='z') {
                'a' as usize
            } else {
                'A' as usize + 26
            };

        counts[idx] += 1;
    }

    ((first, last), counts)
}
