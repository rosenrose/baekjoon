use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    while let (Some(a), Some(b), Some(c)) = (input.next(), input.next(), input.next()) {
        match (a, b, c) {
            (0, 0, 0) => return,
            (a, b, c) if a.max(b).max(c) * 2 >= a + b + c => println!("Invalid"),
            (a, b, c) if a == b && b == c => println!("Equilateral"),
            (a, b, c) if a == b || b == c || c == a => println!("Isosceles"),
            _ => println!("Scalene"),
        }
    }
}
