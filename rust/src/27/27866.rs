use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines();

    let s = input.next().unwrap();
    let i: usize = input.next().unwrap().parse().unwrap();

    println!("{}", s.chars().nth(i - 1).unwrap());
}
