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

    for &ch in input.next().unwrap().as_bytes() {
        let idx = match ch as char {
            'A'..='Z' => ch - b'A' + 1,
            'a'..='z' => ch - b'a' + 27,
            ' ' => 0,
            _ => unreachable!(),
        } as usize;

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
