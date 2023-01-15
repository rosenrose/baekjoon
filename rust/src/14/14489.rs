use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut input = || input.next().unwrap();

    let balance = input() + input();
    let price = input() * 2;

    println!(
        "{}",
        if balance < price {
            balance
        } else {
            balance - price
        }
    );
}
