use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let seconds: i32 = buf.lines().map(|s| s.parse::<i32>().unwrap()).sum();

    println!("{}\n{}", seconds / 60, seconds % 60);
}
