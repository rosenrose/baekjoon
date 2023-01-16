use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();

    for input in buf.lines() {
        if input.is_empty() {
            continue;
        }

        let (lower, upper, digit, blank) = input.chars().fold((0, 0, 0, 0), |(l, u, d, b), ch| {
            if ch.is_lowercase() {
                return (l + 1, u, d, b);
            }
            if ch.is_uppercase() {
                return (l, u + 1, d, b);
            }
            if ch.is_digit(10) {
                return (l, u, d + 1, b);
            }
            if ch.is_whitespace() {
                return (l, u, d, b + 1);
            }

            (l, u, d, b)
        });

        println!("{lower} {upper} {digit} {blank}");
    }
}
