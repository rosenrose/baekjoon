use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.lines().next_back().unwrap();

    let sum: i32 = input.as_bytes().iter().map(|ch| (ch - b'0') as i32).sum();

    println!("{sum}");
}
