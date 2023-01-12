use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());
    let mut input = || input.next().unwrap();

    while let a @ 1.. = input() {
        let leaves = (0..a).fold(1, |leaf, _| (leaf * input()) - input());

        println!("{leaves}");
    }
}
