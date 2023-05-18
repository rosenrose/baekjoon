use std::f64::consts::PI;
use std::fmt;
use std::io;
use std::ops::{Add, Mul, MulAssign, Sub};

const DIGITS: usize = 4;
const POW: u64 = 10_u64.pow(DIGITS as u32);

#[derive(Clone, Copy)]
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

struct BigInt(Vec<u16>);

impl BigInt {
    fn parse(input: &str) -> Self {
        Self(
            input
                .as_bytes()
                .rchunks(DIGITS)
                .map(|chunk| {
                    chunk
                        .iter()
                        .fold(0, |acc, ch| acc * 10 + (ch - b'0') as u16)
                })
                .collect(),
        )
    }

    fn zero_justify(&mut self) {
        while self.0.len() > 1 && self.0.last() == Some(&0) {
            self.0.pop();
        }
    }

    fn fast_fourier_transform(v: &mut Vec<Complex>, is_inverse: bool) {
        Self::bit_reversal(v);

        let mut len = 2;
        let direction = if is_inverse { -1.0 } else { 1.0 };

        while len <= v.len() {
            let angle = -2.0 * PI * direction / len as f64;
            let wlen = Complex(angle.cos(), angle.sin());

            for i in (0..v.len()).step_by(len) {
                let mut w = Complex(1.0, 0.0);
                let half = len >> 1;

                for j in 0..half {
                    let (even, odd) = (v[i + j], v[i + j + half]);

                    v[i + j] = even + odd * w;
                    v[i + j + half] = even - odd * w;

                    w *= wlen;
                }
            }

            len <<= 1;
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
            let mut bit = v.len() >> 1;

            while rev >= bit {
                rev -= bit;
                bit >>= 1;
            }

            rev += bit;

            if i < rev {
                v.swap(i, rev);
            }
        }
    }

    fn normalize(v: Vec<Complex>) -> Self {
        let mut carry = 0;
        let mut result: Vec<_> = v
            .iter()
            .map(|complex| {
                let temp = carry + complex.0.round() as u64;
                carry = temp / POW;

                (temp % POW) as u16
            })
            .collect();

        if carry > 0 {
            result.push(carry as u16);
        }

        Self(result)
    }
}

impl Mul for BigInt {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let len = (self.0.len() + other.0.len()).next_power_of_two();
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

        Self::fast_fourier_transform(&mut a, false);
        Self::fast_fourier_transform(&mut b, false);

        let mut mul: Vec<_> = a.iter().zip(b.iter()).map(|(&a, &b)| a * b).collect();

        Self::fast_fourier_transform(&mut mul, true);

        let mut result = Self::normalize(mul);
        result.zero_justify();

        result
    }
}

impl fmt::Display for BigInt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, num) in self.0.iter().rev().enumerate() {
            if i == 0 {
                write!(f, "{num}").unwrap();
            } else {
                write!(f, "{num:0DIGITS$}").unwrap();
            }
        }

        Ok(())
    }
}

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines().skip(1).map(BigInt::parse);

    let (a, b) = (input.next().unwrap(), input.next().unwrap());

    println!("{}", a * b);
}
