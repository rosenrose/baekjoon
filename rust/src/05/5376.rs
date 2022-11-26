use std::fmt;
use std::io::{stdin, Read};
use std::ops::AddAssign;

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
}

impl AddAssign for Fraction {
    fn add_assign(&mut self, other: Self) {
        let lcm = get_lcm(self.denominator, other.denominator);

        let numerator =
            self.numerator * (lcm / self.denominator) + other.numerator * (lcm / other.denominator);
        let denominator = lcm;

        let gcd = get_gcd(numerator, denominator);

        *self = Self::from(numerator / gcd, denominator / gcd);
    }
}

impl fmt::Display for Fraction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}/{}", self.numerator, self.denominator)
    }
}

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut lines = buf.lines();
    lines.next();

    for line in lines {
        let mut fraction = Fraction::from(0, 1);
        let decimal = line.split('.').next_back().unwrap();

        let mut tokens = decimal.split(['(', ')']);
        let non_repeat = tokens.next().unwrap();

        for (i, c) in non_repeat.char_indices() {
            fraction += Fraction::from(c.to_digit(10).unwrap() as i64, 10_i64.pow(i as u32 + 1));
        }

        if let Some(repeating) = tokens.next() {
            fraction += Fraction::from(
                repeating.parse().unwrap(),
                format!(
                    "{}{}",
                    "9".repeat(repeating.len()),
                    "0".repeat(non_repeat.len())
                )
                .parse()
                .unwrap(),
            );
        }

        println!("{fraction}");
    }
}

fn get_lcm(a: i64, b: i64) -> i64 {
    a / get_gcd(a, b) * b
}

fn get_gcd(mut a: i64, mut b: i64) -> i64 {
    loop {
        (a, b) = (b, a % b);

        if b == 0 {
            return a;
        }
    }
}
