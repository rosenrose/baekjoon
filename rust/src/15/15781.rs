use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());

    let (n, _) = (input.next().unwrap(), input.next());
    let max = (0..n).map(|_| input.next().unwrap()).max().unwrap() + input.max().unwrap();

    println!("{max}");
}
