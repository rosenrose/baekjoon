use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines();
    let mut input = || input.next().unwrap();
    let mut output = String::new();

    let n = parse_int(input());

    'outer: for _ in 0..n {
        let k = parse_int(input());
        let words: Vec<_> = (0..k).map(|_| input()).collect();

        for i in 0..words.len() {
            for j in 0..words.len() {
                if i == j {
                    continue;
                }
                // let merged = format!("{}{}", words[i], words[j]);
                let merged = [words[i], words[j]].concat();

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
