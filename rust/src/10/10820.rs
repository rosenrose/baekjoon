use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    for line in buf.lines() {
        if line.is_empty() {
            continue;
        }

        let (mut lower, mut upper, mut digit, mut blank) = (0, 0, 0, 0);

        line.chars().for_each(|c| {
            if c.is_lowercase() {
                lower += 1;
            }
            if c.is_uppercase() {
                upper += 1;
            }
            if c.is_digit(10) {
                digit += 1;
            }
            if c.is_whitespace() {
                blank += 1;
            }
        });

        println!("{lower} {upper} {digit} {blank}");
    }
}
