use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    while let (Some(a), Some(b), Some(c)) = (input.next(), input.next(), input.next()) {
        let max_diff = (b - a).max(c - b);

        println!("{}", max_diff - 1);
    }
}
