use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();

    let [.., height, width] = [(); 6].map(|_| parse_int(input.next().unwrap()));
    let p_count: usize = input.map(|line| line.matches('P').count()).sum();

    println!("{}", (p_count < height * width) as u8);
}

fn parse_int(buf: &str) -> usize {
    buf.parse().unwrap()
}
