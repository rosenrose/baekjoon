use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();

    let a = input.next().unwrap().to_owned();
    let c = input.fold(a, |acc, b| acc.replace(b, &b.to_lowercase()));

    println!("{c}");
}
