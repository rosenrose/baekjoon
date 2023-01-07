use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
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
