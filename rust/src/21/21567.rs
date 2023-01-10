use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let product: i64 = buf.lines().map(|s| s.parse::<i64>().unwrap()).product();

    for digit in '0'..='9' {
        println!("{}", product.to_string().matches(digit).count());
    }
}
