use std::f64::consts::PI;
use std::io::{stdin, stdout, BufWriter, Read, Write};
use std::ops::{Add, DivAssign, Mul, MulAssign, Sub};

#[derive(Clone, Copy, Debug)]
struct Complex {
    re: f64,
    im: f64,
}

impl Add for Complex {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            re: self.re + other.re,
            im: self.im + other.im,
        }
    }
}
impl Sub for Complex {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            re: self.re - other.re,
            im: self.im - other.im,
        }
    }
}
impl Mul for Complex {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            re: self.re * other.re - self.im * other.im,
            im: self.re * other.im + self.im * other.re,
        }
    }
}
impl MulAssign for Complex {
    fn mul_assign(&mut self, other: Self) {
        *self = Self::mul(*self, other);
    }
}
impl DivAssign for Complex {
    fn div_assign(&mut self, other: Self) {
        let denominator = other.re.powi(2) + other.im.powi(2);

        *self = Self {
            re: (self.re * other.re + self.im * other.im) / denominator,
            im: (self.im * other.re - self.re * other.im) / denominator,
        };
    }
}

fn main() {
    let (stdin, stdout) = (stdin(), stdout());
    let (mut stdin, mut stdout) = (stdin.lock(), BufWriter::new(stdout.lock()));

    let mut buf = String::new();
    stdin.read_to_string(&mut buf).unwrap();

    let mut nums = buf.lines().next().unwrap().split_whitespace();
    let parse_vec = |s: &str| {
        s.chars()
            .rev()
            .map(|c| Complex {
                re: c.to_digit(10).unwrap() as f64,
                im: 0.0,
            })
            .collect::<Vec<_>>()
    };

    let mut a = parse_vec(nums.next().unwrap());
    let mut b = parse_vec(nums.next().unwrap());

    let result = multiply(&mut a, &mut b);

    for r in result.iter().rev() {
        write!(stdout, "{r}").unwrap();
    }
}

fn multiply(a: &mut Vec<Complex>, b: &mut Vec<Complex>) -> Vec<u32> {
    let mut len = 2;

    while len < a.len() + b.len() {
        len *= 2;
    }

    a.resize(len, Complex { re: 0.0, im: 0.0 });
    b.resize(len, Complex { re: 0.0, im: 0.0 });

    fast_fourier_transform(a, false);
    fast_fourier_transform(b, false);

    let mut result: Vec<_> = a.iter().zip(b.iter()).map(|(&a, &b)| a * b).collect();

    fast_fourier_transform(&mut result, true);

    normalize(result)
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
        let direction = Complex {
            re: f64::cos(angle),
            im: f64::sin(angle),
        };

        for i in (0..len).step_by(k * 2) {
            let mut unit = Complex { re: 1.0, im: 0.0 };

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
            *num /= Complex {
                re: len as f64,
                im: 0.0,
            };
        }
    }
}

fn normalize(v: Vec<Complex>) -> Vec<u32> {
    let mut carry = 0;
    let mut result: Vec<_> = v
        .iter()
        .map(|complex| {
            let temp = carry + complex.re.round() as u32;
            carry = temp / 10;

            temp % 10
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
