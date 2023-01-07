use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());
    let mut input = || input.next().unwrap();

    for _ in 0..input() {
        let (n, k) = (input(), input());
        let kids: i32 = (0..n).map(|_| input() / k).sum();

        println!("{kids}");
    }
}
