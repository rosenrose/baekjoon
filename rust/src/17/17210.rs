use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<u32>().unwrap());

    let n = input.next().unwrap();

    if n >= 6 {
        println!("Love is open door");
        return;
    }

    let mut door = input.next().unwrap();

    for _ in 0..n - 1 {
        door = if door == 0 { 1 } else { 0 };
        println!("{door}");
    }
}
