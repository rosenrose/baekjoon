use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    while let (Some(a @ 1..), Some(b @ 1..)) = (input.next(), input.next()) {
        println!(
            "{}",
            match (a, b) {
                (a, b) if a % b == 0 => "multiple",
                (a, b) if b % a == 0 => "factor",
                _ => "neither",
            }
        );
    }
}
