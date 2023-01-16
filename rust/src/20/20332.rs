use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let sum: i32 = input.skip(1).sum();

    println!("{}", if sum % 3 == 0 { "yes" } else { "no" });
}
