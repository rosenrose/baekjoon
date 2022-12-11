use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let result: i64 = buf
        .lines()
        .map(|s| i64::from_str_radix(s, 2).unwrap())
        .product();

    println!("{result:b}");
}
