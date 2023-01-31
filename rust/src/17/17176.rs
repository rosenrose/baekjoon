use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines();
    let mut input = || input.next().unwrap();

    let _n = input();
    let encrypted_count = input().split(' ').fold([0; 53], |mut acc, s| {
        acc[parse_int(s)] += 1;
        acc
    });

    let decrypted_count = input().chars().fold([0; 53], |mut acc, ch| {
        let ch = match ch {
            'A'..='Z' => ch as usize - 'A' as usize + 1,
            'a'..='z' => ch as usize - 'a' as usize + 27,
            ' ' => 0,
            _ => Default::default(),
        };

        acc[ch] += 1;
        acc
    });

    println!(
        "{}",
        if encrypted_count == decrypted_count {
            'y'
        } else {
            'n'
        }
    );
}

fn parse_int(buf: &str) -> usize {
    buf.parse().unwrap()
}
