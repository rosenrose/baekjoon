use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();

    for input in buf.lines().skip(1).map(str::as_bytes) {
        let last = input.iter().last().unwrap() - b'0';

        println!("{}", if last % 2 == 0 { "even" } else { "odd" });
    }
}
