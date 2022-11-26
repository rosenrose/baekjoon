use std::fmt;
use std::ops::{Add, Sub};

#[derive(Copy, Clone)]
struct Fraction {
    numerator: i64,
    denominator: i64,
}

impl Fraction {
    fn new() -> Self {
        Self::from(0, 1)
    }

    fn from(numerator: i64, denominator: i64) -> Self {
        Self {
            numerator,
            denominator,
        }
        .reduced()
    }

    fn from_int(int: i64) -> Self {
        Self::from(int, 1)
    }

    fn reduced(self) -> Self {
        let gcd = get_gcd(self.numerator, self.denominator);
        let (mut numerator, mut denominator) = (self.numerator / gcd, self.denominator / gcd);

        if denominator < 0 {
            numerator *= -1;
            denominator *= -1;
        }

        Self {
            numerator,
            denominator,
        }
    }

    fn multiply(self, mul: i64) -> Self {
        Self::from(self.numerator * mul, self.denominator)
    }

    fn divide(self, div: i64) -> Self {
        Self::from(self.numerator, self.denominator * div)
    }

    fn abs(self) -> Self {
        Self::from(self.numerator.abs(), self.denominator)
    }
}

impl Add for Fraction {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::from(
            self.numerator * other.denominator + other.numerator * self.denominator,
            self.denominator * other.denominator,
        )
    }
}
impl Sub for Fraction {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self::from(
            self.numerator * other.denominator - other.numerator * self.denominator,
            self.denominator * other.denominator,
        )
    }
}

impl fmt::Display for Fraction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.denominator == 1 {
            write!(f, "{}", self.numerator)
        } else {
            write!(f, "{}/{}", self.numerator, self.denominator)
        }
    }
}

fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    let n = parse_int(buf.trim());
    let point_heights: Vec<_> = (0..n + 1)
        .map(|_| {
            read_line(&mut buf);
            parse_int_tuple(&buf)
        })
        .collect();
    read_line(&mut buf);

    let (start_x, end_x) = parse_int_tuple(&buf);
    let (mut start_h, mut end_h) = (Fraction::new(), Fraction::new());

    for i in 0..point_heights.len() - 1 {
        let (x1, h1) = point_heights[i];
        let (x2, h2) = point_heights[i + 1];
        let gradient = Fraction::from(h2 - h1, x2 - x1);

        if (x1..=x2 - 1).contains(&start_x) {
            let delta_x = start_x - x1;
            let delta_y = gradient.multiply(delta_x);

            start_h = Fraction::from_int(h1) + delta_y;
        }
        if (x1 + 1..=x2).contains(&end_x) {
            let delta_x = end_x - x1;
            let delta_y = gradient.multiply(delta_x);

            end_h = Fraction::from_int(h1) + delta_y;
            break;
        }
    }

    let delta_x = end_x - start_x;
    let delta_y = end_h - start_h;
    let avg = delta_y.divide(delta_x);

    println!("{}", avg.abs());
}

fn get_gcd(mut a: i64, mut b: i64) -> i64 {
    loop {
        (a, b) = (b, a % b);

        if b == 0 {
            return a;
        }
    }
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}

fn parse_int(buf: &str) -> i64 {
    buf.parse().unwrap()
}

fn parse_int_tuple(buf: &String) -> (i64, i64) {
    let mut tokens = buf.split_whitespace().map(parse_int);
    (tokens.next().unwrap(), tokens.next().unwrap())
}
