use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    for input in buf.lines().skip(1) {
        println!(
            "{}{}",
            input.chars().nth(0).unwrap().to_uppercase(),
            &input[1..]
        );
    }
}
