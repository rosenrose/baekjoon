use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());
    input.next();

    let x = input.next().unwrap();
    let result = input.filter(|&n| n < x);

    for r in result {
        print!("{r} ");
    }
}
