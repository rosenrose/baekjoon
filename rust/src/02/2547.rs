use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i128>().unwrap());

    for _ in 0..input.next().unwrap() {
        let students = input.next().unwrap();
        let candies: i128 = (0..students).map(|_| input.next().unwrap()).sum();

        println!("{}", if candies % students == 0 { "YES" } else { "NO" });
    }
}
