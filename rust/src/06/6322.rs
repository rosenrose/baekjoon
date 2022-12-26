use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());
    let mut i = 1;

    loop {
        let (name, square) = match (
            input.next().unwrap(),
            input.next().unwrap(),
            input.next().unwrap(),
        ) {
            (-1, b, c) => ("a", c * c - b * b),
            (a, -1, c) => ("b", c * c - a * a),
            (a, b, -1) => ("c", a * a + b * b),
            _ => return,
        };

        println!("Triangle #{i}");

        if square <= 0 {
            println!("Impossible.");
        } else {
            println!("{name} = {:.3}", (square as f64).sqrt());
        }

        println!("");
        i += 1;
    }
}
