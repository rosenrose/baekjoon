use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut input = || input.next().unwrap();

    let (a, b, c, d) = (input(), input(), input(), input());
    println!("{}", (a + d).min(b + c));
}
