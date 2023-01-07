use std::fmt;
use std::io;
use std::ops::Add;

struct Fraction {
    numerator: i32,
    denominator: i32,
}

impl Add for Fraction {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let lcm = get_lcm(self.denominator, other.denominator);

        let numerator =
            self.numerator * (lcm / self.denominator) + other.numerator * (lcm / other.denominator);
        let denominator = lcm;

        let gcd = get_gcd(numerator, denominator);

        Self {
            numerator: numerator / gcd,
            denominator: denominator / gcd,
        }
    }
}

impl fmt::Display for Fraction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.numerator, self.denominator)
    }
}

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());
    let mut input = || input.next().unwrap();

    let fraction1 = Fraction {
        numerator: input(),
        denominator: input(),
    };
    let fraction2 = Fraction {
        numerator: input(),
        denominator: input(),
    };

    println!("{}", fraction1 + fraction2);
}

fn get_lcm(a: i32, b: i32) -> i32 {
    a / get_gcd(a, b) * b
}

fn get_gcd(mut a: i32, mut b: i32) -> i32 {
    loop {
        if b == 0 {
            return a;
        }

        (a, b) = (b, a % b);
    }
}
