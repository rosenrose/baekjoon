use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    for input in buf.lines().skip(1) {
        let first = &input[..1];
        let last = &input[input.len() - 1..];

        println!("{first}{last}");
    }
}
