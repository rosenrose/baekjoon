use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let (index, max) = buf
        .lines()
        .map(|s| s.parse::<i32>().unwrap())
        .enumerate()
        .max_by_key(|&(_, n)| n)
        .unwrap();
    let index = index + 1;

    println!("{max}\n{index}");
}
