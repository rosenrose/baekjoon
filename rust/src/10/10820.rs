use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();

    for input in buf.lines() {
        let mut counts = [0; 4];

        for ch in input.chars() {
            if ch.is_lowercase() {
                counts[0] += 1;
            }
            if ch.is_uppercase() {
                counts[1] += 1;
            }
            if ch.is_digit(10) {
                counts[2] += 1;
            }
            if ch.is_whitespace() {
                counts[3] += 1;
            }
        }

        let [lower, upper, digit, blank] = counts;

        println!("{lower} {upper} {digit} {blank}");
    }
}
