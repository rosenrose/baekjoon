use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines();

    let n: usize = input.next().unwrap().parse().unwrap();
    let s = input.next_back().unwrap();
    let pn = format!("I{}", "OI".repeat(n));

    println!("{}", kmp(s.as_bytes(), pn.as_bytes()));
}

fn kmp(source: &[u8], target: &[u8]) -> i32 {
    let (mut count, mut start) = (0, 0);
    let partial_match = get_partial_match(target);

    for &s in source {
        while target[start] != s && start > 0 {
            start = partial_match[start - 1];
        }

        if target[start] == s {
            if start < target.len() - 1 {
                start += 1;
            } else {
                count += 1;
                start = partial_match[start];
            }
        }
    }

    count
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
