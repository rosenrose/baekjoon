use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();
    let mut input = || input.next().unwrap();

    let (rows, _) = (parse_int(input()), input());
    let (_, _, height, width) = (input(), input(), parse_int(input()), parse_int(input()));

    let p_count: usize = (0..rows).map(|_| input().matches('P').count()).sum();

    println!("{}", u8::from(p_count < height * width));
}

fn parse_int(buf: &str) -> usize {
    buf.parse().unwrap()
}
