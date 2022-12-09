use std::io::{stdin, Read};
use std::{fmt, fmt::Write};

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
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());
    let mut output = String::new();

    loop {
        let (n, d) = (input.next().unwrap(), input.next().unwrap());

        if (n, d) == (0, 0) {
            break;
        }

        writeln!(output, "{}", MixedFraction::from(0, n, d).to_mixed()).unwrap();
    }

    print!("{output}");
}
