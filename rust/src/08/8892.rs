use std::io::{stdin, stdout, BufRead, BufWriter, Write};

fn main() {
    let stdin = stdin();
    let stdout = stdout();
    let mut stdin = stdin.lock();
    let mut stdout = BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();

    let n: i32 = buf.trim().parse().unwrap();

    'outer: for _ in 0..n {
        buf.clear();
        stdin.read_line(&mut buf).unwrap();

        let k: i32 = buf.trim().parse().unwrap();
        let words: Vec<String> = (0..k)
            .map(|_| {
                buf.clear();
                stdin.read_line(&mut buf).unwrap();

                buf.trim().to_string()
            })
            .collect();

        for i in 0..words.len() {
            for j in 0..words.len() {
                if i == j {
                    continue;
                }

                let merged = [&words[i][..], &words[j][..]].concat();

                if is_palindrome(&merged[..]) {
                    writeln!(stdout, "{merged}").unwrap();
                    continue 'outer;
                }
            }
        }

        writeln!(stdout, "0").unwrap();
    }
}

fn is_palindrome(word: &str) -> bool {
    let len = word.len();

    if len <= 1 {
        return true;
    }

    if word.chars().nth(0) != word.chars().nth(len - 1) {
        return false;
    }

    is_palindrome(&word[1..len - 1])
}
