use std::fmt::Write;
use std::io;
use std::ops::Sub;

#[derive(Copy, Clone)]
struct Fraction(i64, i64);

impl Fraction {
    fn reduced(numerator: i64, denominator: i64) -> Self {
        let gcd = get_gcd(numerator, denominator).abs();

        Self(numerator / gcd, denominator / gcd)
    }
}

impl Sub for Fraction {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self::reduced(self.0 * other.1 - other.0 * self.1, self.1 * other.1)
    }
}

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i64>);
    let mut output = String::new();

    while let (Some(m @ 2..), Some(n @ 2..)) = (input.next(), input.next()) {
        let mut fraction = Fraction(m, n);

        while fraction.0 > 1 {
            for i in (fraction.1 / fraction.0) + 1.. {
                let next = fraction - Fraction(1, i);

                if next.0 > 0 && next.1 < 1_000_000 {
                    fraction = next;
                    write!(output, "{i} ").unwrap();
                    break;
                }
            }
        }

        writeln!(output, "{}", fraction.1).unwrap();
    }

    print!("{output}");
}

fn get_gcd(mut a: i64, mut b: i64) -> i64 {
    loop {
        if b == 0 {
            return a;
        }

        (a, b) = (b, a % b);
    }
}
