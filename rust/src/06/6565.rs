use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();

    for input in buf.lines() {
        let (left, c) = input.split_once('=').unwrap();
        let (a, b) = left.split_once('+').unwrap();

        println!(
            "{}",
            if parse_int(a) + parse_int(b) == parse_int(c) {
                "True"
            } else {
                "False"
            }
        );
    }
}

fn parse_int(buf: &str) -> i32 {
    buf.chars().rev().collect::<String>().parse().unwrap()
}
