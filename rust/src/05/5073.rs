use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());

    loop {
        match (
            input.next().unwrap(),
            input.next().unwrap(),
            input.next().unwrap(),
        ) {
            (0, 0, 0) => return,
            (a, b, c) if a.max(b).max(c) * 2 >= a + b + c => println!("Invalid"),
            (a, b, c) if a == b && b == c => println!("Equilateral"),
            (a, b, c) if a == b || b == c || c == a => println!("Isosceles"),
            _ => println!("Scalene"),
        }
    }
}
