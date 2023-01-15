use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut input = || input.next().unwrap();
    let mut output = String::new();

    for _ in 0..input() {
        let (mut price, options) = (input(), input());

        price += (0..options).map(|_| input() * input()).sum::<i32>();

        writeln!(output, "{price}").unwrap();
    }

    print!("{output}");
}
