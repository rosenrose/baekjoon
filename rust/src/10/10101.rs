use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines().flat_map(str::parse::<i32>);

    println!(
        "{}",
        match [(); 3].map(|_| input.next().unwrap()) {
            [a, b, c] if a + b + c != 180 => "Error",
            [a, b, c] if a == b && b == c => "Equilateral",
            [a, b, c] if a == b || b == c || c == a => "Isosceles",
            _ => "Scalene",
        }
    );
}
