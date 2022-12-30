use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    for input in buf.lines().skip(1) {
        println!("{}", "=".repeat(input.parse().unwrap()));
    }
}
