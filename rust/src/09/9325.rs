use std::fmt::Write;
use std::io::{stdin, Read};

fn main() {
    let stdin = stdin();
    let mut stdin = stdin.lock();

    let mut buf = String::new();
    stdin.read_to_string(&mut buf).unwrap();

    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());
    let mut output = String::new();

    for _ in 0..input.next().unwrap() {
        let mut price = input.next().unwrap();
        let options = input.next().unwrap();

        price += (0..options)
            .map(|_| input.next().unwrap() * input.next().unwrap())
            .sum::<i32>();

        writeln!(output, "{price}").unwrap();
    }

    print!("{output}");
}
