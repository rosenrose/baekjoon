use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines().map(|s| s.parse::<i32>().unwrap());

    let burgers: Vec<_> = (0..3).map(|_| input.next().unwrap()).collect();
    let drinks = (0..2).map(|_| input.next().unwrap());

    let prices = drinks.flat_map(|drink| burgers.iter().map(move |burger| burger + drink));

    println!("{}", prices.min().unwrap() - 50);
}
