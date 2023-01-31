use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut input = || input.next().unwrap();

    while let (a @ 1.., b @ 1.., c @ 1..) = (input(), input(), input()) {
        println!(
            "{}",
            match (a, b, c) {
                (a, b, c) if a.max(b).max(c) * 2 >= a + b + c => "Invalid",
                (a, b, c) if a == b && b == c => "Equilateral",
                (a, b, c) if a == b || b == c || c == a => "Isosceles",
                _ => "Scalene",
            }
        );
    }
}
