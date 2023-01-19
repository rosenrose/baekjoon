use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let (_, x) = (input.next(), input.next().unwrap());

    for num in input.filter(|&n| n < x) {
        print!("{num} ");
    }
}
