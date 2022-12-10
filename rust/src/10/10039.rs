use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let sum: i32 = buf
        .lines()
        .map(|s| (s.parse::<i32>().unwrap()).max(40))
        .sum();

    println!("{}", sum / 5);
}
