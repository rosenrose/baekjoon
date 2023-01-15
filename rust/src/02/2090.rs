use std::io;
use std::ops::Add;

struct Fraction(i64, i64);

impl Fraction {
    fn reduced(numerator: i64, denominator: i64) -> Self {
        let gcd = get_gcd(numerator, denominator).abs();

        Self(numerator / gcd, denominator / gcd)
    }
}

impl Add for Fraction {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::reduced(self.0 * other.1 + other.0 * self.1, self.1 * other.1)
    }
}

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.split_ascii_whitespace().flat_map(str::parse::<i64>);

    let harmonic_mean = input
        .skip(1)
        .fold(Fraction(0, 1), |acc, n| acc + Fraction(1, n));

    println!("{}/{}", harmonic_mean.1, harmonic_mean.0);
}

fn get_gcd(mut a: i64, mut b: i64) -> i64 {
    loop {
        if b == 0 {
            return a;
        }

        (a, b) = (b, a % b);
    }
}
