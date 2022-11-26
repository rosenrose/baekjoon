use std::fmt;
use std::io::{stdin, stdout, BufWriter, Read, Write};

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

    fn parse(input: &str) -> Self {
        match parse_int_vec(input)[..] {
            [a, b] => Self::from(0, a, b),
            _ => Self::from(0, 0, 1),
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
    let (stdin, stdout) = (stdin(), stdout());
    let (mut stdin, mut stdout) = (stdin.lock(), BufWriter::new(stdout.lock()));

    let mut buf = String::new();
    stdin.read_to_string(&mut buf).unwrap();

    for line in buf.lines() {
        if line == "0 0" {
            return;
        }

        writeln!(stdout, "{}", MixedFraction::parse(line).to_mixed()).unwrap();
    }
}

fn parse_int_vec(buf: &str) -> Vec<i32> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}
