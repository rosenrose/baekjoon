use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let product: i32 = buf.lines().flat_map(str::parse::<i32>).product();

    for digit in '0'..='9' {
        println!("{}", product.to_string().matches(digit).count());
    }
}
