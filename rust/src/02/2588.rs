use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines();

    let (a, b) = (parse_int(input.next().unwrap()), input.next().unwrap());
    let result = b.chars().rev().map(|ch| ch.to_digit(10).unwrap() * a);

    for r in result {
        println!("{r}");
    }
    println!("{}", a * parse_int(b));
}

fn parse_int(buf: &str) -> u32 {
    buf.parse().unwrap()
}
