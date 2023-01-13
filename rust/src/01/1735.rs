use std::fmt;
use std::io;
use std::ops::Add;

struct Fraction(i32, i32);

impl Add for Fraction {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let lcm = get_lcm(self.1, other.1);

        let numerator = self.0 * (lcm / self.1) + other.0 * (lcm / other.1);
        let denominator = lcm;

        let gcd = get_gcd(numerator, denominator);

        Self(numerator / gcd, denominator / gcd)
    }
}

impl fmt::Display for Fraction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.0, self.1)
    }
}

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());
    let mut input = || input.next().unwrap();

    let fraction1 = Fraction(input(), input());
    let fraction2 = Fraction(input(), input());

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
