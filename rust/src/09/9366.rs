use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());
    let mut input = || input.next().unwrap();

    for i in 1..=input() {
        print!("Case #{i}: ");

        match (input(), input(), input()) {
            (a, b, c) if a.max(b).max(c) * 2 >= a + b + c => println!("invalid!"),
            (a, b, c) if a == b && b == c => println!("equilateral"),
            (a, b, c) if a == b || b == c || c == a => println!("isosceles"),
            _ => println!("scalene"),
        }
    }
}
