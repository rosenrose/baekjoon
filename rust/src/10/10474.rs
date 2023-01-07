use std::fmt::{self, Write};
use std::io;

struct MixedFraction {
    integer: i32,
    numerator: i32,
    denominator: i32,
}

impl MixedFraction {
    fn from(integer: i32, numerator: i32, denominator: i32) -> Self {
        Self {
            integer,
            numerator,
            denominator,
        }
    }

    fn to_mixed(self) -> Self {
        Self::from(
            self.numerator / self.denominator,
            self.numerator % self.denominator,
            self.denominator,
        )
    }
}

impl fmt::Display for MixedFraction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} {} / {}",
            self.integer, self.numerator, self.denominator
        )
    }
}

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());
    let mut output = String::new();

    while let (Some(n), Some(d)) = (input.next(), input.next()) {
        if (n, d) == (0, 0) {
            break;
        }

        writeln!(output, "{}", MixedFraction::from(0, n, d).to_mixed()).unwrap();
    }

    print!("{output}");
}
