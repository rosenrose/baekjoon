use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    input.next();

    let v = input.next_back().unwrap();
    let count = input.filter(|&n| n == v).count();

    println!("{count}");
}
