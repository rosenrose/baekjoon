use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines();
    input.next();

    let encrypted_count = input
        .next()
        .unwrap()
        .split(' ')
        .fold([0; 53], |mut acc, s| {
            acc[parse_int(s)] += 1;
            acc
        });

    let decrypted_count = input.next().unwrap().chars().fold([0; 53], |mut acc, ch| {
        let ch = match ch {
            'A'..='Z' => ch as u8 - 'A' as u8 + 1,
            'a'..='z' => ch as u8 - 'a' as u8 + 27,
            _ => 0,
        };

        acc[ch as usize] += 1;
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
