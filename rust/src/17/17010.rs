use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();
    let mut input = || input.next().unwrap();

    for (n, x) in (0..parse_int(input())).map(|_| (parse_int(input()), input())) {
        println!("{}", x.repeat(n));
    }
}

fn parse_int(buf: &str) -> usize {
    buf.parse().unwrap()
}
