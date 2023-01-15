use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    while let (Some(m @ 1..), Some(f @ 1..)) = (input.next(), input.next()) {
        println!("{}", m + f);
    }
}
