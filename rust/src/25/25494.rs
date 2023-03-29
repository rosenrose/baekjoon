use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    for _ in 0..input.next().unwrap() {
        println!("{}", input.by_ref().take(3).min().unwrap());
    }
}
