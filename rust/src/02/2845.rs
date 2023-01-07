use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());

    let (l, p) = (input.next().unwrap(), input.next().unwrap());
    let exact_count = l * p;

    for count in input {
        print!("{} ", count - exact_count);
    }
}
