use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines();

    let s: String = input.next().unwrap().matches(char::is_alphabetic).collect();
    let k = input.next().unwrap();

    println!("{}", u8::from(s.contains(k)));
}
