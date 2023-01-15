use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let points: Vec<_> = (0..4).map(|_| input.next().unwrap()).collect();
    let x = input.next().unwrap();

    let number = match points.iter().position(|&p| p == x) {
        Some(i) => i + 1,
        None => 0,
    };

    println!("{number}");
}
