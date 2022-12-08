use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    for (i, input) in buf.lines().skip(1).enumerate() {
        println!("{}. {input}", i + 1);
    }
}
