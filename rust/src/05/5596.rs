use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let s: i32 = (0..4).map(|_| input.next().unwrap()).sum();
    let t: i32 = input.sum();

    println!("{}", s.max(t));
}
