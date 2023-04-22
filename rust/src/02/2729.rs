use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();

    let n: i32 = input.next().unwrap().parse().unwrap();
    let mut input = input.flat_map(|s| i128::from_str_radix(s, 2));

    for (a, b) in (0..n).map(|_| (input.next().unwrap(), input.next().unwrap())) {
        println!("{:b}", a + b);
    }
}
