use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf.split_ascii_whitespace();
    let mut input = || input.next().unwrap();

    let (s1, s2) = (input().parse().unwrap(), input().parse().unwrap());

    if (0..s1).any(|_| input() != input()) {
        println!("Wrong Answer");
        return;
    }

    if (0..s2).any(|_| input() != input()) {
        println!("Why Wrong!!!");
        return;
    }

    println!("Accepted");
}
