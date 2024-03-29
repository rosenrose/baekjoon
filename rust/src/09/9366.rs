use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    for i in 1..=input.next().unwrap() {
        println!(
            "Case #{i}: {}",
            match [(); 3].map(|_| input.next().unwrap()) {
                [a, b, c] if a.max(b).max(c) * 2 >= a + b + c => "invalid!",
                [a, b, c] if a == b && b == c => "equilateral",
                [a, b, c] if a == b || b == c || c == a => "isosceles",
                _ => "scalene",
            }
        );
    }
}
