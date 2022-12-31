use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf.split_ascii_whitespace();

    let a = input.next().unwrap().to_string();
    let c = input.fold(a, |acc, b| acc.replace(b, &b.to_lowercase()));

    println!("{c}");
}
