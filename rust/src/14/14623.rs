use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let result: i64 = buf
        .lines()
        .map(|s| i64::from_str_radix(s, 2).unwrap())
        .product();

    println!("{result:b}");
}
