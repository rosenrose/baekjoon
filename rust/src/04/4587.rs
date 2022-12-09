use std::fmt::Write;
use std::io::{stdin, Read};
use std::ops::Sub;

#[derive(Copy, Clone)]
struct Fraction {
    numerator: i64,
    denominator: i64,
}

impl Fraction {
    fn from(numerator: i64, denominator: i64) -> Self {
        Self {
            numerator,
            denominator,
        }
    }

    fn unit(int: i64) -> Self {
        Self::from(1, int)
    }

    fn reduced(self) -> Self {
        let gcd = get_gcd(self.numerator, self.denominator).abs();

        Self {
            numerator: self.numerator / gcd,
            denominator: self.denominator / gcd,
        }
    }
}

impl Sub for Fraction {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self::from(
            self.numerator * other.denominator - other.numerator * self.denominator,
            self.denominator * other.denominator,
        )
        .reduced()
    }
}

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i64>().unwrap());
    let mut output = String::new();

    loop {
        let (m, n) = (input.next().unwrap(), input.next().unwrap());

        if (m, n) == (0, 0) {
            break;
        }

        let mut fraction = Fraction::from(m, n);

        while fraction.numerator > 1 {
            for i in (fraction.denominator / fraction.numerator) + 1.. {
                let next = fraction - Fraction::unit(i);

                if next.numerator > 0 && next.denominator < 1_000_000 {
                    fraction = next;
                    write!(output, "{i} ").unwrap();
                    break;
                }
            }
        }

        writeln!(output, "{}", fraction.denominator).unwrap();
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
