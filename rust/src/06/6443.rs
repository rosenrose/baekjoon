use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut output = String::new();

    for input in buf.lines().skip(1) {
        let mut word = input.as_bytes().to_vec();
        word.sort();

        loop {
            writeln!(output, "{}", String::from_utf8(word.clone()).unwrap()).unwrap();

            if !next_permutation(&mut word) {
                break;
            }
        }
    }

    print!("{output}");
}

fn next_permutation(chars: &mut Vec<u8>) -> bool {
    let len = chars.len();

    let Some(i) = (1..len).rfind(|&i| chars[i - 1] < chars[i]) else {
        return false;
    };
    let j = (i..len).rfind(|&j| chars[j] > chars[i - 1]).unwrap();

    chars.swap(i - 1, j);
    chars[i..].sort();

    true
}
