use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
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
