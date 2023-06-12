use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();

    let [_, _, _, _, height, width] = [(); 6].map(|_| parse_int(input.next().unwrap()));
    let p_count: usize = input.map(|line| line.matches('P').count()).sum();

    println!("{}", u8::from(p_count < height * width));
}

fn parse_int(buf: &str) -> usize {
    buf.parse().unwrap()
}
