use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    for input in buf.lines().skip(1) {
        let first = &input[..1];
        let last = &input[input.len() - 1..];

        println!("{first}{last}");
    }
}
