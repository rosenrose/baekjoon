use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());

    let (l, p) = (input.next().unwrap(), input.next().unwrap());
    let exact_count = l * p;

    for count in input {
        print!("{} ", count - exact_count);
    }
}
