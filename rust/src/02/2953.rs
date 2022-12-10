use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());

    let (number, score) = (1..=5)
        .map(|i| {
            let sum: i32 = (0..4).map(|_| input.next().unwrap()).sum();
            (i, sum)
        })
        .max_by_key(|&(_, score)| score)
        .unwrap();

    println!("{number} {score}");
}
