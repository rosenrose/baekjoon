use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();

    for input in buf.lines().skip(1) {
        let first = input.chars().nth(0).unwrap();
        let last = input.chars().last().unwrap();

        println!("{first}{last}");
    }
}
