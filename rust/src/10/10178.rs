use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());

    for _ in 0..input.next().unwrap() {
        let (c, v) = (input.next().unwrap(), input.next().unwrap());

        println!(
            "You get {} piece(s) and your dad gets {} piece(s).",
            c / v,
            c % v
        );
    }
}
