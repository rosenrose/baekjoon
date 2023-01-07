use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    println!("{}", buf.lines().count());
}
