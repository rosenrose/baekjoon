use std::io::{stdin, stdout, BufRead, BufWriter, Write};

fn main() {
    let stdin = stdin();
    let stdout = stdout();
    let mut stdin = stdin.lock();
    let mut stdout = BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();

    let n: i32 = buf.trim().parse().unwrap();

    for _ in 0..n {
        buf.clear();
        stdin.read_line(&mut buf).unwrap();

        let mut count = 0;
        let result = if is_palindrome(buf.trim(), &mut count) {
            1
        } else {
            0
        };

        writeln!(stdout, "{result} {count}").unwrap();
    }
}

fn is_palindrome(word: &str, count: &mut i32) -> bool {
    *count += 1;

    let len = word.len();
    if len <= 1 {
        return true;
    }

    if word.chars().nth(0) != word.chars().nth(len - 1) {
        return false;
    }

    is_palindrome(&word[1..len - 1], count)
}

/*
let mut count = 0;
let mut result = 1;
let (mut i, mut j) = (0, word.len() - 1);

loop {
    count += 1;

    if i >= j {
        break;
    }

    if word.chars().nth(i) != word.chars().nth(j) {
        result = 0;
        break;
    }

    i += 1;
    j -= 1;
}

println!("{} {}", result, count);
*/
