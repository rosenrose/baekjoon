use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf.split_ascii_whitespace();
    let (ah, require) = (input.next().unwrap(), input.next().unwrap());

    println!(
        "{}",
        if ah.len() >= require.len() {
            "go"
        } else {
            "no"
        }
    );
}
