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
    fn from(args: Vec<i64>) -> Self {
        let (a, b, c, d) = (args[0], args[1], args[2], args[3]);
        Self { a, b, c, d }
    }
}
impl Add for Real {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::from(normalize(
            self.a * other.a,
            other.a * self.b + self.a * other.b,
            other.a * self.c + self.a * other.c,
            self.d,
        ))
    }
}
impl Sub for Real {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self::from(normalize(
            self.a * other.a,
            other.a * self.b - self.a * other.b,
            other.a * self.c - self.a * other.c,
            self.d,
        ))
    }
}
impl Mul for Real {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self::from(normalize(
            self.a * other.a,
            self.b * other.b + self.c * other.c * self.d,
            other.b * self.c + self.b * other.c,
            self.d,
        ))
    }
}
impl Div for Real {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Self::from(normalize(
            self.a * (other.b.pow(2) - other.c.pow(2) * self.d),
            other.a * (self.b * other.b - self.c * other.c * self.d),
            other.a * (other.b * self.c - self.b * other.c),
            self.d,
        ))
    }
}

impl fmt::Display for Real {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {} {}", self.a, self.b, self.c, self.d)
    }
}

fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    let a = Real::from(parse_int_vec(&buf));
    read_line(&mut buf);

    let b = Real::from(parse_int_vec(&buf));

    println!("{}\n{}\n{}\n{}", a + b, a - b, a * b, a / b);
}

fn normalize(mut a: i64, mut b: i64, mut c: i64, mut d: i64) -> Vec<i64> {
    if c == 0 {
        d = 0;
    }
    if d == 0 {
        c = 0;
    }

    let gcd = get_gcd([a, b, c].into_iter().filter(|&i| i != 0));
    (a, b, c) = (a / gcd, b / gcd, c / gcd);

    if a < 0 {
        a *= -1;
        b *= -1;
        c *= -1;
    }

    vec![a, b, c, d]
}

fn get_gcd<I>(nums: I) -> i64
where
    I: Iterator<Item = i64>,
{
    nums.reduce(|mut a, mut b| loop {
        (a, b) = (b, a % b);

        if b == 0 {
            return a;
        }
    })
    .unwrap()
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}

fn parse_int_vec(buf: &String) -> Vec<i64> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}
