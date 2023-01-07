use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();

    let n: usize = input.next().unwrap().parse().unwrap();
    let s = input.next_back().unwrap().as_bytes();

    let pn = format!("I{}", "OI".repeat(n));
    let pn = pn.as_bytes();
    let partial_match = get_partial_match(&pn);

    let (mut count, mut start) = (0, 0);

    for &s_char in s {
        while pn[start] != s_char && start > 0 {
            start = partial_match[start - 1];
        }

        if pn[start] == s_char {
            if start < pn.len() - 1 {
                start += 1;
            } else {
                count += 1;
                start = partial_match[start];
            }
        }
    }

    println!("{count}");
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
