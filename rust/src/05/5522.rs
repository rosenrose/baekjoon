use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let sum: i32 = buf.lines().map(|input| input.parse::<i32>().unwrap()).sum();

    println!("{sum}");
}
