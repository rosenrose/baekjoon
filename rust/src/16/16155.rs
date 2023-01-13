use std::fmt;
use std::io;
use std::ops::{Add, Sub};

#[derive(Copy, Clone)]
struct Fraction(i64, i64);

impl Fraction {
    fn new() -> Self {
        Self(0, 1)
    }

    fn from_int(int: i64) -> Self {
        Self(int, 1)
    }

    fn reduced(numerator: i64, denominator: i64) -> Self {
        let gcd = get_gcd(numerator, denominator).abs();

        Self(numerator / gcd, denominator / gcd)
    }

    fn multiply(self, mul: i64) -> Self {
        Self::reduced(self.0 * mul, self.1)
    }

    fn divide(self, div: i64) -> Self {
        Self::reduced(self.0, self.1 * div)
    }

    fn abs(self) -> Self {
        Self(self.0.abs(), self.1)
    }
}

impl Add for Fraction {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::reduced(self.0 * other.1 + other.0 * self.1, self.1 * other.1)
    }
}
impl Sub for Fraction {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self::reduced(self.0 * other.1 - other.0 * self.1, self.1 * other.1)
    }
}

impl fmt::Display for Fraction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.1 == 1 {
            write!(f, "{}", self.0)
        } else {
            write!(f, "{}/{}", self.0, self.1)
        }
    }
}

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i64>().unwrap());
    let mut input = || input.next().unwrap();

    let n = input();
    let point_heights: Vec<_> = (0..n + 1).map(|_| (input(), input())).collect();

    let (start_x, end_x) = (input(), input());
    let (mut start_h, mut end_h) = (Fraction::new(), Fraction::new());

    for i in 1..point_heights.len() {
        let (x1, h1) = point_heights[i - 1];
        let (x2, h2) = point_heights[i];
        let gradient = Fraction(h2 - h1, x2 - x1);

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
