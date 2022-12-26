use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let input = buf.lines().map(|s| s.parse::<i32>().unwrap());

    match input.collect::<Vec<_>>()[..] {
        [a, b, c] if a + b + c != 180 => println!("Error"),
        [a, b, c] if a == b && b == c => println!("Equilateral"),
        [a, b, c] if a == b || b == c || c == a => println!("Isosceles"),
        _ => println!("Scalene"),
    }
}
