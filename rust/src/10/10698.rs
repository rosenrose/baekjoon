use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();
    let mut input = || input.next().unwrap();

    for i in 1..=parse_int(input()) {
        let (x, op, y, _, z) = (
            parse_int(input()),
            input(),
            parse_int(input()),
            input(),
            parse_int(input()),
        );

        println!(
            "Case {i}: {}",
            match op {
                "+" if x + y == z => "YES",
                "-" if x - y == z => "YES",
                _ => "NO",
            }
        );
    }
}

fn parse_int(buf: &str) -> i32 {
    buf.parse().unwrap()
}
