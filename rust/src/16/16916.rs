use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();

    let (s, p) = (input.next().unwrap(), input.next().unwrap());

    println!("{}", u8::from(s.contains(p)));
}
