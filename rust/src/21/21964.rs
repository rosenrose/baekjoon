use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let input = buf.lines().next_back().unwrap();

    println!("{}", &input[input.len() - 5..]);
}
