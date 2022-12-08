use std::fmt;
use std::io::{stdin, Read};
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
        let gcd = get_gcd(self.numerator, self.denominator).abs();

        Self {
            numerator: self.numerator / gcd,
            denominator: self.denominator / gcd,
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
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i64>().unwrap());

    let n = input.next().unwrap();
    let point_heights: Vec<_> = (0..n + 1)
        .map(|_| (input.next().unwrap(), input.next().unwrap()))
        .collect();

    let (start_x, end_x) = (input.next().unwrap(), input.next().unwrap());
    let (mut start_h, mut end_h) = (Fraction::new(), Fraction::new());

    for i in 1..point_heights.len() {
        let (x1, h1) = point_heights[i - 1];
        let (x2, h2) = point_heights[i];
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

    let (delta_x, delta_y) = (end_x - start_x, end_h - start_h);
    let avg = delta_y.divide(delta_x);

    println!("{}", avg.abs());
}

fn get_gcd(mut a: i64, mut b: i64) -> i64 {
    loop {
        if b == 0 {
            return a;
        }

        (a, b) = (b, a % b);
    }
}
