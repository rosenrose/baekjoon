use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf.lines().next_back().unwrap().to_string();

    while input.matches('s').count() != input.matches('t').count() {
        input.remove(0);
    }

    println!("{input}");
}
