use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.split_ascii_whitespace();
    let mut output = String::new();

    for word in input.skip(1) {
        let mut count = 0;
        let result = if is_palindrome(word.as_bytes(), &mut count) {
            1
        } else {
            0
        };

        writeln!(output, "{result} {count}").unwrap();
    }

    print!("{output}");
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
