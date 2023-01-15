use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let date = input.next().unwrap();
    let count = input.filter(|&num| num == date).count();

    println!("{count}");
}
