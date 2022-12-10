use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let product = buf
        .lines()
        .map(|s| s.parse::<i32>().unwrap())
        .product::<i32>()
        .to_string();

    for digit in '0'..='9' {
        let count = product.matches(digit).count();

        println!("{count}");
    }
}
