use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();

    for (i, input) in buf.lines().enumerate().skip(1) {
        let words: Vec<_> = input.split(' ').rev().collect();

        println!("Case #{i}: {}", words.join(" "));
    }
}
