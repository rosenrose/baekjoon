use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    for input in buf.lines().skip(1) {
        println!("{}", "=".repeat(input.parse().unwrap()));
    }
}
