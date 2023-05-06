use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines();

    let _n = input.next();
    let mut encrypted_count = [0; 53];

    for num in input.next().unwrap().split(' ').map(parse_int) {
        encrypted_count[num] += 1;
    }

    let mut decrypted_count = [0; 53];

    for ch in input.next().unwrap().chars() {
        let idx = match ch {
            'A'..='Z' => ch as usize - 'A' as usize + 1,
            'a'..='z' => ch as usize - 'a' as usize + 27,
            ' ' => 0,
            _ => unreachable!(),
        };

        decrypted_count[idx] += 1;
    }

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
