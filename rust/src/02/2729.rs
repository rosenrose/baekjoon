use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();
    let mut input = || input.next().unwrap();

    let n: i32 = input().parse().unwrap();

    for _ in 0..n {
        let (a, b) = (
            i128::from_str_radix(input(), 2).unwrap(),
            i128::from_str_radix(input(), 2).unwrap(),
        );

        println!("{:b}", a + b);
    }
}
