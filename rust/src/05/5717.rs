use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());

    while let (Some(m), Some(f)) = (input.next(), input.next()) {
        if (m, f) == (0, 0) {
            return;
        }

        println!("{}", m + f);
    }
}
