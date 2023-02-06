use std::f64::consts::PI;
use std::fmt::Write;
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

struct BigInt(Vec<Complex>);

impl BigInt {
    fn parse(input: &str) -> Self {
        Self(
            input
                .as_bytes()
                .rchunks(2)
                .map(|chunk| {
                    let mut exp = 1;

                    Complex(
                        chunk.iter().rev().fold(0.0, |acc, &ch| {
                            let num = (ch as i32 - '0' as i32) * exp;
                            exp *= 10;

                            acc + num as f64
                        }),
                        0.0,
                    )
                })
                .collect(),
        )
    }

    fn mul(mut self, mut other: Self) -> Vec<i32> {
        let mut len = 2;

        while len < self.0.len() + other.0.len() {
            len *= 2;
        }

        self.0.resize(len, Complex(0.0, 0.0));
        other.0.resize(len, Complex(0.0, 0.0));

        fast_fourier_transform(&mut self.0, false);
        fast_fourier_transform(&mut other.0, false);

        let mut result: Vec<_> = self
            .0
            .iter()
            .zip(other.0.iter())
            .map(|(&a, &b)| a * b)
            .collect();

        fast_fourier_transform(&mut result, true);

        normalized(result)
    }
}

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let mut input = buf.split_whitespace().map(BigInt::parse);
    let mut output = String::new();

    let (a, b) = (input.next().unwrap(), input.next().unwrap());
    let result = a.mul(b);

    for (i, num) in result.iter().rev().enumerate() {
        if i == 0 {
            write!(output, "{num}").unwrap();
        } else {
            write!(output, "{num:02}").unwrap();
        }
    }

    print!("{output}");
}

fn fast_fourier_transform(v: &mut Vec<Complex>, is_inverse: bool) {
    let len = v.len();
    let mut j = 0;

    for i in 1..len {
        let mut bit = len / 2;

        while j >= bit {
            j -= bit;
            bit /= 2;
        }

        j += bit;

        if i < j {
            v.swap(i, j);
        }
    }

    let mut k = 1;

    while k < len {
        let angle = (if is_inverse { PI } else { -PI }) / k as f64;
        let direction = Complex(f64::cos(angle), f64::sin(angle));

        for i in (0..len).step_by(k * 2) {
            let mut unit = Complex(1.0, 0.0);

            for j in 0..k {
                let even = v[i + j];
                let odd = v[i + j + k] * unit;

                v[i + j] = even + odd;
                v[i + j + k] = even - odd;

                unit *= direction;
            }
        }

        k *= 2;
    }

    if is_inverse {
        for num in v.iter_mut() {
            (*num).div(len as f64);
        }
    }
}

fn normalized(v: Vec<Complex>) -> Vec<i32> {
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
