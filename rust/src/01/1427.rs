use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();

    let mut chars = buf.trim().as_bytes().to_vec();
    chars.sort_by_key(|&ch| std::cmp::Reverse(ch));

    println!("{}", String::from_utf8(chars).unwrap());
}
