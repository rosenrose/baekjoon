use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines();
    let mut output = String::new();

    let (t, p) = (input.next().unwrap(), input.next().unwrap());
    let p_chars: Vec<_> = p.chars().collect();

    let partial_match = get_partial_match(&p_chars);
    // println!("{partial_match:?}");

    let (mut count, mut start) = (0, 0);
    let mut indices = Vec::new();

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

    writeln!(output, "{count}").unwrap();

    for i in indices {
        write!(output, "{} ", i + 1).unwrap();
    }

    print!("{output}");
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
