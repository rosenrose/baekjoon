use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    for line in buf.split('\n') {
        if line.is_empty() || line == "\r" {
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
            if c == ' ' {
                blank += 1;
            }
        });

        println!("{lower} {upper} {digit} {blank}");
    }
}
