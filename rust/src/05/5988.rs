use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();

    for input in buf.lines().skip(1) {
        let last = input.chars().last().unwrap() as u8 - '0' as u8;

        println!("{}", if last % 2 == 0 { "even" } else { "odd" });
    }
}
