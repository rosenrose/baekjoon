use std::io::{stdin, stdout, BufRead, BufWriter, Write};

fn main() {
    let (stdin, stdout) = (stdin(), stdout());
    let (mut stdin, mut stdout) = (stdin.lock(), BufWriter::new(stdout.lock()));

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();

    let t = buf.replace(['\r', '\n'], "");
    buf.clear();
    stdin.read_line(&mut buf).unwrap();

    let p = buf.replace(['\r', '\n'], "");
    let p_chars: Vec<char> = p.chars().collect();

    let partial_match = get_partial_match(&p_chars);
    // println!("{partial_match:?}");

    let mut count = 0;
    let mut indices = Vec::new();
    let mut start = 0;

    for (index, t_char) in t.char_indices() {
        while p_chars[start] != t_char && start > 0 {
            start = partial_match[start - 1];
        }

        if p_chars[start] == t_char {
            if start < p.len() - 1 {
                start += 1;
            } else {
                count += 1;
                indices.push(index + 1 - p.len());

                start = partial_match[start];
            }
        }
    }

    writeln!(stdout, "{count}").unwrap();

    for i in indices {
        write!(stdout, "{} ", i + 1).unwrap();
    }
}

fn get_partial_match(chars: &Vec<char>) -> Vec<usize> {
    let mut partial_match = vec![0; chars.len()];
    let mut start = 0;

    for i in 1..chars.len() {
        while (chars[start] != chars[i]) && start > 0 {
            start = partial_match[start - 1];
        }

        if chars[start] == chars[i] {
            start += 1;
            partial_match[i] = start;
        }
    }

    partial_match
}
