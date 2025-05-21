use std::io;

const MAX: usize = 1_000_000 * 2 + 1;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines();

    let n: usize = input.next().unwrap().parse().unwrap();
    let s = input.next_back().unwrap();
    let pn = format!("I{}", "OI".repeat(n));

    println!("{}", kmp(s.as_bytes(), pn.as_bytes()));
}

fn kmp(source: &[u8], target: &[u8]) -> i32 {
    let mut partial_match = [0; MAX];
    let mut i = 0;

    for j in 1..target.len() {
        while (target[i] != target[j]) && i > 0 {
            i = partial_match[i - 1];
        }

        if target[i] == target[j] {
            i += 1;
            partial_match[j] = i;
        }
    }

    i = 0;
    let mut count = 0;

    for &s in source {
        while target[i] != s && i > 0 {
            i = partial_match[i - 1];
        }

        if target[i] == s {
            if i < target.len() - 1 {
                i += 1;
            } else {
                count += 1;
                i = partial_match[i];
            }
        }
    }

    count
}
