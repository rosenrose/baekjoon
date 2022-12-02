use std::fmt;
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

        let gcd = get_gcd([a, b, c].into_iter().filter(|&i| i != 0));
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
        Self {
            a: self.a * other.a,
            b: other.a * self.b + self.a * other.b,
            c: other.a * self.c + self.a * other.c,
            d: self.d,
        }
    }
}
impl Sub for Real {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            a: self.a * other.a,
            b: other.a * self.b - self.a * other.b,
            c: other.a * self.c - self.a * other.c,
            d: self.d,
        }
    }
}
impl Mul for Real {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            a: self.a * other.a,
            b: self.b * other.b + self.c * other.c * self.d,
            c: other.b * self.c + self.b * other.c,
            d: self.d,
        }
    }
}
impl Div for Real {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Self {
            a: self.a * (other.b.pow(2) - other.c.pow(2) * self.d),
            b: other.a * (self.b * other.b - self.c * other.c * self.d),
            c: other.a * (other.b * self.c - self.b * other.c),
            d: self.d,
        }
    }
}

#[derive(Copy, Clone)]
struct Complex {
    re: Real,
    im: Real,
}

impl Complex {
    fn from(re: Real, im: Real) -> Self {
        Self { re, im }
    }

    fn parse(input: &str) -> Self {
        match parse_int_vec(input)[..] {
            [a0, b0, c0, d0, a1, b1, c1, d1] => {
                Self::from(Real::from(a0, b0, c0, d0), Real::from(a1, b1, c1, d1))
            }
            _ => Self::from(Real::from(1, 0, 0, 0), Real::from(1, 0, 0, 0)),
        }
    }
}
impl Add for Complex {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            re: (self.re + other.re).normalized(),
            im: (self.im + other.im).normalized(),
        }
    }
}
impl Sub for Complex {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            re: (self.re - other.re).normalized(),
            im: (self.im - other.im).normalized(),
        }
    }
}
impl Mul for Complex {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            re: (self.re * other.re - self.im * other.im).normalized(),
            im: (self.re * other.im + self.im * other.re).normalized(),
        }
    }
}
impl Div for Complex {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        let divisor = other.re * other.re + other.im * other.im;

        Self {
            re: ((self.re * other.re + self.im * other.im) / divisor).normalized(),
            im: ((self.im * other.re - self.re * other.im) / divisor).normalized(),
        }
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
    let mut buf = String::new();
    read_line(&mut buf);

    let a = Complex::parse(buf.trim());
    read_line(&mut buf);

    let b = Complex::parse(buf.trim());

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

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}

fn parse_int_vec(buf: &str) -> Vec<i64> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}
