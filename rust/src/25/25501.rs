use std::io::{stdin, stdout, BufWriter, Read, Write};

fn main() {
    let (stdin, stdout) = (stdin(), stdout());
    let (mut stdin, mut stdout) = (stdin.lock(), BufWriter::new(stdout.lock()));

    let mut buf = String::new();
    stdin.read_to_string(&mut buf).unwrap();

    buf.lines().skip(1).for_each(|line| {
        let mut count = 0;
        let result = if is_palindrome(line.as_bytes(), &mut count) {
            1
        } else {
            0
        };

        writeln!(stdout, "{result} {count}").unwrap();
    })
}

fn is_palindrome(word: &[u8], count: &mut i32) -> bool {
    *count += 1;

    let len = word.len();

    if len <= 1 {
        return true;
    }

    if word[0] != word[len - 1] {
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
