use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());

    while let (Some(m), Some(f)) = (input.next(), input.next()) {
        if (m, f) == (0, 0) {
            return;
        }

        println!("{}", m + f);
    }
}
