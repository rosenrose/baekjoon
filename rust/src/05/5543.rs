use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines().flat_map(str::parse::<i32>);

    let burgers: Vec<_> = input.by_ref().take(3).collect();
    let prices = input.flat_map(|drink| burgers.iter().map(move |burger| burger + drink));

    println!("{}", prices.min().unwrap() - 50);
}
