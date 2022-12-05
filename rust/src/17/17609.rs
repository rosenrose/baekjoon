use std::io::{stdin, Read};

fn main() {
    let stdin = stdin();
    let mut stdin = stdin.lock();

    let mut buf = String::new();
    stdin.read_to_string(&mut buf).unwrap();

    buf.split_ascii_whitespace().skip(1).for_each(|line| {
        let (is_palin, is_pseudo_palin) = is_palindrome(line);

        println!(
            "{}",
            if is_palin {
                0
            } else {
                if is_pseudo_palin {
                    1
                } else {
                    2
                }
            }
        );
    });
}

fn is_palindrome(word: &str) -> (bool, bool) {
    let len = word.len();

    if len <= 1 {
        return (true, false);
    }

    let (mut i, mut j) = (0, len - 1);
    let bytes = word.as_bytes();

    loop {
        if i >= j {
            break;
        }

        if bytes[i] == bytes[j] {
            i += 1;
            j -= 1;
            continue;
        }

        if is_palindrome_recur(&bytes[i + 1..=j]) || is_palindrome_recur(&bytes[i..=j - 1]) {
            return (false, true);
        }

        return (false, false);
    }

    (true, false)
}

fn is_palindrome_recur(bytes: &[u8]) -> bool {
    let len = bytes.len();

    if len <= 1 {
        return true;
    }

    if bytes[0] != bytes[len - 1] {
        return false;
    }

    is_palindrome_recur(&bytes[1..len - 1])
}
