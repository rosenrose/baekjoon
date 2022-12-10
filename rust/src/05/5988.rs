use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    for input in buf.lines().skip(1) {
        let last = input.chars().last().unwrap().to_digit(10).unwrap();

        println!("{}", if last % 2 == 0 { "even" } else { "odd" });
    }
}
