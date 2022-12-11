use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    for (i, input) in buf.lines().enumerate().skip(1) {
        let words: Vec<_> = input.split(' ').rev().collect();

        println!("Case #{i}: {}", words.join(" "));
    }
}
