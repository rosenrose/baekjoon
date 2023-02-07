use std::f64::consts::PI;
use std::fmt::{self, Write};
use std::ops::{Add, Mul, MulAssign, Sub};

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

struct BigInt(Vec<i32>);

impl BigInt {
    fn parse(input: &str) -> Self {
        Self(
            input
                .as_bytes()
                .rchunks(2)
                .map(|chunk| {
                    let mut exp = 1;

                    chunk.iter().rev().fold(0, |acc, &ch| {
                        let num = (ch as i32 - '0' as i32) * exp;
                        exp *= 10;

                        acc + num
                    })
                })
                .collect(),
        )
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

        for i in 0..len {
            if let Some(&num) = self.0.get(i) {
                a[i] = Complex(num as f64, 0.0);
            }
            if let Some(&num) = other.0.get(i) {
                b[i] = Complex(num as f64, 0.0);
            }
        }

        fast_fourier_transform(&mut a, false);
        fast_fourier_transform(&mut b, false);

        let mut result: Vec<_> = a.iter().zip(b.iter()).map(|(&a, &b)| a * b).collect();

        fast_fourier_transform(&mut result, true);

        Self(normalize(result))
    }
}

impl fmt::Display for BigInt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output = String::new();

        for (i, num) in self.0.iter().rev().enumerate() {
            if i == 0 {
                write!(output, "{num}").unwrap();
            } else {
                write!(output, "{num:02}").unwrap();
            }
        }

        write!(f, "{output}")
    }
}

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let mut input = buf.split_whitespace().map(BigInt::parse);
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

fn normalize(v: Vec<Complex>) -> Vec<i32> {
    let mut carry = 0;
    let mut result: Vec<_> = v
        .iter()
        .map(|complex| {
            let temp = carry + complex.0.round() as i32;
            carry = temp / 100;

            temp % 100
        })
        .collect();

    if carry > 0 {
        result.push(carry);
    }

    while result.len() > 1 && *result.last().unwrap() == 0 {
        result.pop();
    }

    result
}
