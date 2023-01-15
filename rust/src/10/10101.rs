use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.lines().flat_map(str::parse::<i32>);

    match input.collect::<Vec<_>>()[..] {
        [a, b, c] if a + b + c != 180 => println!("Error"),
        [a, b, c] if a == b && b == c => println!("Equilateral"),
        [a, b, c] if a == b || b == c || c == a => println!("Isosceles"),
        _ => println!("Scalene"),
    }
}
