use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    for (i, input) in buf.lines().enumerate().skip(1) {
        println!("{i}. {input}");
    }
}
