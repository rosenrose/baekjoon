use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());

    let n = input.next().unwrap();
    let (odds, evens) = input.fold((0, 0), |(odd, even), num| {
        if num % 2 == 0 {
            (odd, even + 1)
        } else {
            (odd + 1, even)
        }
    });

    let is_possible = if n % 2 == 0 {
        odds == evens
    } else {
        odds == evens + 1
    };

    println!("{}", if is_possible { 1 } else { 0 });
}
