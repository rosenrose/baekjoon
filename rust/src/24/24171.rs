use std::fmt;
use std::io;
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
        Self { a, b, c, d }
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

#[derive(Copy, Clone)]
struct Complex {
    re: Real,
    im: Real,
}

impl Complex {
    fn from(re: Real, im: Real) -> Self {
        Self {
            re: re.normalized(),
            im: im.normalized(),
        }
    }
}
impl Add for Complex {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::from(self.re + other.re, self.im + other.im)
    }
}
impl Sub for Complex {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self::from(self.re - other.re, self.im - other.im)
    }
}
impl Mul for Complex {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self::from(
            self.re * other.re - self.im * other.im,
            self.re * other.im + self.im * other.re,
        )
    }
}
impl Div for Complex {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        let divisor = other.re * other.re + other.im * other.im;

        Self::from(
            (self.re * other.re + self.im * other.im) / divisor,
            (self.im * other.re - self.re * other.im) / divisor,
        )
    }
}

impl fmt::Display for Real {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {} {}", self.a, self.b, self.c, self.d)
    }
}
impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.re, self.im)
    }
}

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i64>().unwrap());
    let mut input = || input.next().unwrap();

    let a = Complex::from(
        Real::from(input(), input(), input(), input()),
        Real::from(input(), input(), input(), input()),
    );
    let b = Complex::from(
        Real::from(input(), input(), input(), input()),
        Real::from(input(), input(), input(), input()),
    );

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
