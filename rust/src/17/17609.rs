use std::io::{stdin, stdout, BufWriter, Read, Write};

fn main() {
    let (stdin, stdout) = (stdin(), stdout());
    let (mut stdin, mut stdout) = (stdin.lock(), BufWriter::new(stdout.lock()));

    let mut buf = String::new();
    stdin.read_to_string(&mut buf).unwrap();

    buf.lines().skip(1).for_each(|line| {
        let (is_palin, is_pseudo_palin) = is_palindrome(line);

        writeln!(
            stdout,
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
        )
        .unwrap();
    });
}

fn is_palindrome(word: &str) -> (bool, bool) {
    let len = word.len();

    if len <= 1 {
        return (true, false);
    }

    let (mut i, mut j) = (0, len - 1);
    let chars: Vec<_> = word.chars().collect();

    loop {
        if i >= j {
            break;
        }

        if chars[i] == chars[j] {
            i += 1;
            j -= 1;
            continue;
        }

        if is_palindrome_recur(&chars[i + 1..=j]) || is_palindrome_recur(&chars[i..=j - 1]) {
            return (false, true);
        }

        return (false, false);
    }

    (true, false)
}

fn is_palindrome_recur(chars: &[char]) -> bool {
    let len = chars.len();

    if len <= 1 {
        return true;
    }

    if chars[0] != chars[len - 1] {
        return false;
    }

    is_palindrome_recur(&chars[1..len - 1])
}
