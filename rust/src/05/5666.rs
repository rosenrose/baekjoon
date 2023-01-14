use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<f32>().unwrap());

    while let (Some(h), Some(p)) = (input.next(), input.next()) {
        println!("{:.2}", h / p);
    }
}
