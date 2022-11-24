use std::fmt;
use std::io::{stdin, Read};
use std::ops::AddAssign;

struct Fraction {
    numerator: i64,
    denominator: i64,
}

impl Fraction {
    fn new(numerator: i64, denominator: i64) -> Self {
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

        *self = Self {
            numerator: numerator / gcd,
            denominator: denominator / gcd,
        }
    }
}

impl fmt::Display for Fraction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} / {}", self.numerator, self.denominator)
    }
}

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    for line in buf.lines() {
        let mut number = line.split('.');
        let integer = Fraction::new(number.next().unwrap().parse().unwrap(), 1);
        let decimal = number.next().unwrap();

        let mut tokens = decimal.split(['(', ')']);
        let non_repeat = tokens.next().unwrap();
        let repeating = tokens.next().unwrap();

        let mut fraction = integer;

        for (i, c) in non_repeat.char_indices() {
            fraction += Fraction::new(c.to_digit(10).unwrap() as i64, 10_i64.pow(i as u32 + 1));
        }

        fraction += Fraction::new(
            repeating.parse().unwrap(),
            format!(
                "{}{}",
                "9".repeat(repeating.len()),
                "0".repeat(non_repeat.len())
            )
            .parse()
            .unwrap(),
        );

        println!("{line} = {fraction}");
    }
}

fn get_lcm(a: i64, b: i64) -> i64 {
    (a * b) / get_gcd(a, b)
}

fn get_gcd(mut a: i64, mut b: i64) -> i64 {
    loop {
        (a, b) = (b, a % b);

        if b == 0 {
            return a;
        }
    }
}