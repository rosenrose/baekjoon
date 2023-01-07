use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());

    for _ in 0..input.next().unwrap() {
        let n = input.next().unwrap();
        let sum: i32 = (0..n).map(|_| input.next().unwrap()).sum();

        println!("{sum}");
    }
}
