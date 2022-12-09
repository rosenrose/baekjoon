use std::fmt::Write;
use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf.split_ascii_whitespace();
    let mut output = String::new();

    let n = parse_int(input.next().unwrap());

    'outer: for _ in 0..n {
        let k = parse_int(input.next().unwrap());
        let words: Vec<_> = (0..k).map(|_| input.next().unwrap()).collect();

        for i in 0..words.len() {
            for j in 0..words.len() {
                if i == j {
                    continue;
                }

                // let merged = format!("{}{}", words[i], words[j]);
                let merged = [&words[i][..], &words[j][..]].concat();

                if is_palindrome(merged.as_bytes()) {
                    writeln!(output, "{merged}").unwrap();
                    continue 'outer;
                }
            }
        }

        writeln!(output, "0").unwrap();
    }

    print!("{output}");
}

fn is_palindrome(word: &[u8]) -> bool {
    let len = word.len();

    if len <= 1 {
        return true;
    }

    let (mut i, mut j) = (0, len - 1);

    loop {
        if i >= j {
            break;
        }

        if word[i] != word[j] {
            return false;
        }

        i += 1;
        j -= 1;
    }

    true
}

fn parse_int(buf: &str) -> i32 {
    buf.parse().unwrap()
}
