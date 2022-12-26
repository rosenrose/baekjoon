use std::fmt;
use std::io::{stdin, Read};
use std::ops::{Add, Div, Mul, Sub};

#[derive(Copy, Clone)]
struct Real {
    a: i64,
    b: i64,
    c: i64,
    d: i64,
}

impl Real {
    fn from(a: i64, b: i64, c: i64, d: i64) -> Self {
        Self { a, b, c, d }.normalized()
    }

    fn normalized(self) -> Self {
        let (mut a, mut b, mut c, mut d) = (self.a, self.b, self.c, self.d);

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

        Self { a, b, c, d }
    }
}
impl Add for Real {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::from(
            self.a * other.a,
            other.a * self.b + self.a * other.b,
            other.a * self.c + self.a * other.c,
            self.d,
        )
    }
}
impl Sub for Real {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self::from(
            self.a * other.a,
            other.a * self.b - self.a * other.b,
            other.a * self.c - self.a * other.c,
            self.d,
        )
    }
}
impl Mul for Real {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self::from(
            self.a * other.a,
            self.b * other.b + self.c * other.c * self.d,
            other.b * self.c + self.b * other.c,
            self.d,
        )
    }
}
impl Div for Real {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Self::from(
            self.a * (other.b.pow(2) - other.c.pow(2) * self.d),
            other.a * (self.b * other.b - self.c * other.c * self.d),
            other.a * (other.b * self.c - self.b * other.c),
            self.d,
        )
    }
}

impl fmt::Display for Real {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {} {}", self.a, self.b, self.c, self.d)
    }
}

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i64>().unwrap());
    let mut input = || input.next().unwrap();

    let a = Real::from(input(), input(), input(), input());
    let b = Real::from(input(), input(), input(), input());

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
