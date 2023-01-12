use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines();
    let mut output = String::new();

    let (t, p) = (input.next().unwrap(), input.next().unwrap());
    let (count, indices) = kmp_with_index(t.as_bytes(), p.as_bytes());

    writeln!(output, "{count}").unwrap();

    for i in indices {
        write!(output, "{} ", i + 1).unwrap();
    }

    print!("{output}");
}

fn kmp_with_index(source: &[u8], target: &[u8]) -> (i32, Vec<usize>) {
    let (mut count, mut start) = (0, 0);
    let mut indices = Vec::new();

    let partial_match = get_partial_match(target);
    // println!("{partial_match:?}");
    for (i, &s) in source.iter().enumerate() {
        while target[start] != s && start > 0 {
            start = partial_match[start - 1];
        }

        if target[start] == s {
            if start < target.len() - 1 {
                start += 1;
            } else {
                count += 1;
                indices.push(i + 1 - target.len());

                start = partial_match[start];
            }
        }
    }

    (count, indices)
}

fn get_partial_match(chars: &[u8]) -> Vec<usize> {
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
