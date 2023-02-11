use std::cmp::Ordering;
use std::collections::VecDeque;
use std::fmt;
use std::io;

const DIGITS: usize = 18;
const EXP: i128 = 10_i128.pow(DIGITS as u32);

#[derive(PartialEq)]
struct BigInt {
    abs: VecDeque<i64>,
    sign: i8,
}

impl BigInt {
    fn new() -> Self {
        Self {
            abs: VecDeque::from([0]),
            sign: 1,
        }
    }

    fn from(abs: VecDeque<i64>, sign: i8) -> Self {
        Self { abs, sign }
    }

    fn parse(input: &str) -> Self {
        let mut result = Self {
            abs: input
                .as_bytes()
                .rchunks(DIGITS)
                .map(|chunk| {
                    let mut exp = 1;

                    chunk.iter().rev().fold(0, |acc, &ch| {
                        if ch as char == '-' {
                            return acc;
                        }

                        let num = (ch as i64 - '0' as i64) * exp;
                        exp *= 10;

                        acc + num
                    })
                })
                .collect(),
            sign: if input.starts_with('-') { -1 } else { 1 },
        };

        result.zero_justify();
        result
    }

    fn is_zero(&self) -> bool {
        self.abs.iter().all(|&i| i == 0)
    }

    fn zero_justify(&mut self) {
        while self.abs.len() > 1 && self.abs.back() == Some(&0) {
            self.abs.pop_back();
        }

        if self.is_zero() {
            self.sign = 1;
        }
    }

    fn len(&self) -> usize {
        self.abs.len()
    }

    fn abs(&self) -> Self {
        Self::from(self.abs.clone(), 1)
    }

    fn add(&self, other: &Self) -> Self {
        if self.sign * other.sign == -1 {
            return if self.sign == 1 {
                self.sub(&other.abs())
            } else {
                other.sub(&self.abs())
            };
        }

        let mut carry = 0;
        let mut sum: VecDeque<_> = (0..self.len().max(other.len()))
            .map(|i| {
                let temp = carry
                    + *self.abs.get(i).unwrap_or(&0) as i128
                    + *other.abs.get(i).unwrap_or(&0) as i128;
                carry = temp / EXP;

                (temp % EXP) as i64
            })
            .collect();

        if carry > 0 {
            sum.push_back(carry as i64);
        }

        Self::from(sum, self.sign)
    }

    fn sub(&self, other: &Self) -> Self {
        if self.sign * other.sign == -1 {
            return if self.sign == 1 {
                self.add(&other.abs())
            } else {
                let mut result = other.add(&self.abs());
                result.sign = -1;

                result
            };
        }

        if self < other {
            let mut result = other.sub(self);
            result.sign *= -1;

            return result;
        }

        let mut carry = 0;
        let diff: VecDeque<_> = (0..self.len().max(other.len()))
            .map(|i| {
                let temp = carry + self.abs.get(i).unwrap_or(&0) - other.abs.get(i).unwrap_or(&0);

                if temp < 0 {
                    carry = -1;
                    temp + EXP as i64
                } else {
                    carry = 0;
                    temp
                }
            })
            .collect();

        let mut result = Self::from(diff, self.sign);

        result.zero_justify();
        result
    }

    fn mul_int(&self, other: i64) -> Self {
        let mut carry = 0;
        let mut result: VecDeque<_> = self
            .abs
            .iter()
            .map(|&num| {
                let temp = carry + num as i128 * other.abs() as i128;
                carry = temp / EXP;

                (temp % EXP) as i64
            })
            .collect();

        if carry > 0 {
            result.push_back(carry as i64);
        }

        Self::from(result, self.sign * other.signum() as i8)
    }

    fn mul(&self, other: &Self) -> Self {
        if self.is_zero() || other.is_zero() {
            return Self::new();
        }
        if self.len() == 1 {
            return other.mul_int(self.abs[0] * self.sign as i64);
        }
        if other.len() == 1 {
            return self.mul_int(other.abs[0] * other.sign as i64);
        }
        // a = x * 10^m + y
        // b = w * 10^m + z
        // ab = xw * 10^2m + (xz + yw) * 10^m + yz
        let m = self.len().max(other.len()) / 2;
        let mut half = m.min(self.len());

        let y = Self::from(self.abs.range(0..half).copied().collect(), 1);
        let x = if half < self.len() {
            Self::from(self.abs.range(half..).copied().collect(), 1)
        } else {
            Self::new()
        };

        half = m.min(other.len());

        let z = Self::from(other.abs.range(0..half).copied().collect(), 1);
        let w = if half < other.len() {
            Self::from(other.abs.range(half..).copied().collect(), 1)
        } else {
            Self::new()
        };

        let (mut xw, yz) = (x.mul(&w), y.mul(&z));
        let r = x.add(&y).mul(&w.add(&z));
        let mut xz_plus_yw = r.sub(&xw.add(&yz));

        if !xw.is_zero() {
            for _ in 0..2 * m {
                xw.abs.push_front(0);
            }
        }
        if !xz_plus_yw.is_zero() {
            for _ in 0..m {
                xz_plus_yw.abs.push_front(0);
            }
        }

        Self::from((xw.add(&xz_plus_yw).add(&yz)).abs, self.sign * other.sign)
    }
}

impl PartialOrd for BigInt {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.len() == other.len() {
            Some(self.abs.iter().rev().cmp(other.abs.iter().rev()))
        } else {
            Some(self.len().cmp(&other.len()))
        }
    }
}

impl fmt::Display for BigInt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.sign == -1 {
            write!(f, "-").unwrap();
        }

        self.abs.iter().rev().enumerate().for_each(|(i, num)| {
            if i == 0 {
                write!(f, "{num}").unwrap();
            } else {
                write!(f, "{num:0DIGITS$}").unwrap();
            }
        });

        write!(f, "")
    }
}

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines().map(BigInt::parse);
    let (a, b) = (input.next().unwrap(), input.next().unwrap());

    println!("{}\n{}\n{}", a.add(&b), a.sub(&b), a.mul(&b));
}
