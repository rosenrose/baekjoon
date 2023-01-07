use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());

    for i in 1..=input.next().unwrap() {
        let (a, b) = (input.next().unwrap(), input.next().unwrap());

        println!("Case #{i}: {a} + {b} = {}", a + b);
    }
}
