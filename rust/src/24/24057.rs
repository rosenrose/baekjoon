use std::fmt;
use std::io;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Copy, Clone)]
struct Real(i64, i64, i64, i64);

impl Real {
    fn reduced(&self) -> Self {
        let (mut a, mut b, mut c, mut d) = (self.0, self.1, self.2, self.3);

        if c == 0 {
            d = 0;
        }

        let gcd = get_gcd([a, b, c].into_iter());
        (a, b, c) = (a / gcd, b / gcd, c / gcd);

        if a < 0 {
            a *= -1;
            b *= -1;
            c *= -1;
        }

        Self(a, b, c, d)
    }
}
impl Add for Real {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(
            self.0 * other.0,
            other.0 * self.1 + self.0 * other.1,
            other.0 * self.2 + self.0 * other.2,
            self.3,
        )
        .reduced()
    }
}
impl Sub for Real {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self(
            self.0 * other.0,
            other.0 * self.1 - self.0 * other.1,
            other.0 * self.2 - self.0 * other.2,
            self.3,
        )
        .reduced()
    }
}
impl Mul for Real {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self(
            self.0 * other.0,
            self.1 * other.1 + self.2 * other.2 * self.3,
            other.1 * self.2 + self.1 * other.2,
            self.3,
        )
        .reduced()
    }
}
impl Div for Real {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Self(
            self.0 * (other.1.pow(2) - other.2.pow(2) * self.3),
            other.0 * (self.1 * other.1 - self.2 * other.2 * self.3),
            other.0 * (other.1 * self.2 - self.1 * other.2),
            self.3,
        )
        .reduced()
    }
}

impl fmt::Display for Real {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {} {}", self.0, self.1, self.2, self.3)
    }
}

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i64>().unwrap());
    let mut input = || input.next().unwrap();

    let a = Real(input(), input(), input(), input());
    let b = Real(input(), input(), input(), input());

    println!("{}\n{}\n{}\n{}", a + b, a - b, a * b, a / b);
}

fn get_gcd<I>(nums: I) -> i64
where
    I: Iterator<Item = i64>,
{
    nums.reduce(|mut a, mut b| loop {
        if b == 0 {
            return a;
        }

        (a, b) = (b, a % b);
    })
    .unwrap()
}
