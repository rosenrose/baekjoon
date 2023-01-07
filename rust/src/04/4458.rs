use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    for input in buf.lines().skip(1) {
        println!(
            "{}{}",
            input.chars().nth(0).unwrap().to_uppercase(),
            &input[1..]
        );
    }
}
