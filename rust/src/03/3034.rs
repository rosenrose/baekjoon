use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<f64>().unwrap());
    input.next();

    let (w, h) = (input.next().unwrap(), input.next().unwrap());

    for len in input {
        println!(
            "{}",
            if len <= (w * w + h * h).sqrt() {
                "DA"
            } else {
                "NE"
            }
        );
    }
}
