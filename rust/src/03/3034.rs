use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<f64>().unwrap());
    input.next();

    let (w, h) = (input.next().unwrap(), input.next().unwrap());

    for len in input {
        println!(
            "{}",
            if len <= (w * w + h * h).sqrt() {
                "DA"
            } else {
                "NE"
            }
        );
    }
}