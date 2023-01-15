use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let n = input.next().unwrap();
    let count: i32 = input.map(|fav| fav.min(n)).sum();

    println!("{count}");
}
