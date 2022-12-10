use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf.lines().map(|s| s.parse::<i32>().unwrap());

    let burgers: Vec<_> = (0..3).map(|_| input.next().unwrap()).collect();
    let drinks = (0..2).map(|_| input.next().unwrap());

    let prices = drinks.flat_map(|drink| burgers.iter().map(move |burger| burger + drink));

    println!("{}", prices.min().unwrap() - 50);
}
