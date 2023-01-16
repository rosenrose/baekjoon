use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();

    for input in buf.lines() {
        let (lower, upper, digit, blank) = input.chars().fold((0, 0, 0, 0), |mut acc, ch| {
            if ch.is_lowercase() {
                acc.0 += 1;
            }
            if ch.is_uppercase() {
                acc.1 += 1;
            }
            if ch.is_digit(10) {
                acc.2 += 1;
            }
            if ch.is_whitespace() {
                acc.3 += 1;
            }

            acc
        });

        println!("{lower} {upper} {digit} {blank}");
    }
}
