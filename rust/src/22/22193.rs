use std::f64::consts::PI;
use std::fmt::{self, Write};
use std::io;
use std::ops::{Add, Mul, MulAssign, Sub};

const DIGITS: usize = 3;
const EXP: i64 = 10_i64.pow(DIGITS as u32);

#[derive(Clone, Copy, Debug)]
struct Complex(f64, f64);

impl Complex {
    fn div(&mut self, num: f64) {
        self.0 /= num;
        self.1 /= num;
    }
}

impl Add for Complex {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1)
    }
}
impl Sub for Complex {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self(self.0 - other.0, self.1 - other.1)
    }
}
impl Mul for Complex {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self(
            self.0 * other.0 - self.1 * other.1,
            self.0 * other.1 + self.1 * other.0,
        )
    }
}
impl MulAssign for Complex {
    fn mul_assign(&mut self, other: Self) {
        *self = *self * other;
    }
}

struct BigInt(Vec<i64>);

impl BigInt {
    fn parse(input: &str) -> Self {
        Self(
            input
                .as_bytes()
                .rchunks(DIGITS)
                .map(|chunk| {
                    let mut exp = 1;

                    chunk.iter().rev().fold(0, |acc, &ch| {
                        let num = (ch as i64 - '0' as i64) * exp;
                        exp *= 10;

                        acc + num
                    })
                })
                .collect(),
        )
    }

    fn zero_justify(&mut self) {
        while self.0.len() > 1 && self.0.last() == Some(&0) {
            self.0.pop();
        }
    }
}

impl Mul for BigInt {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let mut len = 2;

        while len < self.0.len() + other.0.len() {
            len *= 2;
        }

        let mut a = vec![Complex(0.0, 0.0); len];
        let mut b = vec![Complex(0.0, 0.0); len];

        for i in 0..self.0.len().max(other.0.len()) {
            if let Some(&num) = self.0.get(i) {
                a[i] = Complex(num as f64, 0.0);
            }
            if let Some(&num) = other.0.get(i) {
                b[i] = Complex(num as f64, 0.0);
            }
        }

        fast_fourier_transform(&mut a, false);
        fast_fourier_transform(&mut b, false);

        let mut mul: Vec<_> = a.iter().zip(b.iter()).map(|(&a, &b)| a * b).collect();

        fast_fourier_transform(&mut mul, true);

        let mut result = Self(normalize(mul));

        result.zero_justify();
        result
    }
}

impl fmt::Display for BigInt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output = String::new();

        for (i, num) in self.0.iter().rev().enumerate() {
            if i == 0 {
                write!(output, "{num}").unwrap();
            } else {
                write!(output, "{num:0DIGITS$}").unwrap();
            }
        }

        write!(f, "{output}")
    }
}

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines().skip(1).map(BigInt::parse);

    let (a, b) = (input.next().unwrap(), input.next().unwrap());

    println!("{}", a * b);
}

fn fast_fourier_transform(v: &mut Vec<Complex>, is_inverse: bool) {
    bit_reversal(v);

    let mut len = 2;
    let direction = if is_inverse { -1.0 } else { 1.0 };

    while len <= v.len() {
        let angle = -2.0 * PI * direction / len as f64;
        let wlen = Complex(angle.cos(), angle.sin());

        for i in (0..v.len()).step_by(len) {
            let mut w = Complex(1.0, 0.0);

            for j in 0..len / 2 {
                let (even, odd) = (v[i + j], v[i + j + len / 2]);

                v[i + j] = even + odd * w;
                v[i + j + len / 2] = even - odd * w;

                w *= wlen;
            }
        }

        len *= 2;
    }

    if is_inverse {
        len = v.len();

        for num in v.iter_mut() {
            (*num).div(len as f64);
        }
    }
}

fn bit_reversal(v: &mut Vec<Complex>) {
    let mut rev = 0;

    for i in 1..v.len() {
        let mut bit = v.len() / 2;

        while rev >= bit {
            rev -= bit;
            bit /= 2;
        }

        rev += bit;

        if i < rev {
            v.swap(i, rev);
        }
    }
}

fn normalize(v: Vec<Complex>) -> Vec<i64> {
    let mut carry = 0;
    let mut result: Vec<_> = v
        .iter()
        .map(|complex| {
            let temp = carry + complex.0.round() as i64;
            carry = temp / EXP;

            temp % EXP
        })
        .collect();

    if carry > 0 {
        result.push(carry);
    }

    result
}
