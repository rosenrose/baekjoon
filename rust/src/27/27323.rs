use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines().flat_map(str::parse::<i32>);

    let (a, b) = (input.next().unwrap(), input.next().unwrap());
    println!("{}", a * b);
}
