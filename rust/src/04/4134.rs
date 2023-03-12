use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.lines().flat_map(str::parse::<u32>);
    let mut output = String::new();

    for n in input.skip(1) {
        let prime = (n..).find(|&num| is_prime(num)).unwrap();
        writeln!(output, "{prime}").unwrap();
    }

    print!("{output}");
}

fn is_prime(num: u32) -> bool {
    if num <= 1 {
        return false;
    }

    (2..).take_while(|i| i * i <= num).all(|i| num % i != 0)
}
