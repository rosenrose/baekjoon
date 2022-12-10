use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i128>().unwrap());

    for _ in 0..input.next().unwrap() {
        let students = input.next().unwrap();
        let candies: i128 = (0..students).map(|_| input.next().unwrap()).sum();

        println!("{}", if candies % students == 0 { "YES" } else { "NO" });
    }
}
