use std::collections::VecDeque;
use std::fmt;
use std::io;
use std::ops::{Add, Mul, Sub};

const DIGITS: usize = 18;
const EXP: i128 = 10_i128.pow(DIGITS as u32);

#[derive(PartialEq)]
struct BigInt(VecDeque<i64>);

impl BigInt {
    fn new() -> Self {
        Self(VecDeque::from([0]))
    }

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

    fn is_zero(&self) -> bool {
        self.0.iter().all(|&i| i == 0)
    }

    fn zero_justify(&mut self) {
        while self.0.len() > 1 && self.0.back() == Some(&0) {
            self.0.pop_back();
        }
    }

    fn mul_int(&self, other: i64) -> Self {
        let mut carry = 0;
        let mut result: VecDeque<_> = self
            .0
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

        Self(result)
    }
}

impl Add for &BigInt {
    type Output = BigInt;

    fn add(self, other: Self) -> Self::Output {
        let mut carry = 0;
        let mut sum: VecDeque<_> = (0..self.0.len().max(other.0.len()))
            .map(|i| {
                let temp = carry
                    + *self.0.get(i).unwrap_or(&0) as i128
                    + *other.0.get(i).unwrap_or(&0) as i128;
                carry = temp / EXP;

                (temp % EXP) as i64
            })
            .collect();

        if carry > 0 {
            sum.push_back(carry as i64);
        }

        BigInt(sum)
    }
}
impl Sub for &BigInt {
    type Output = BigInt;

    fn sub(self, other: Self) -> Self::Output {
        let mut carry = 0;
        let diff: VecDeque<_> = (0..self.0.len().max(other.0.len()))
            .map(|i| {
                let temp = carry + self.0.get(i).unwrap_or(&0) - other.0.get(i).unwrap_or(&0);

                if temp < 0 {
                    carry = -1;
                    temp + EXP as i64
                } else {
                    carry = 0;
                    temp
                }
            })
            .collect();

        let mut result = BigInt(diff);

        result.zero_justify();
        result
    }
}
impl Mul for &BigInt {
    type Output = BigInt;

    fn mul(self, other: Self) -> Self::Output {
        if self.is_zero() || other.is_zero() {
            return BigInt::new();
        }
        if self.0.len() == 1 {
            return other.mul_int(self.0[0]);
        }
        if other.0.len() == 1 {
            return self.mul_int(other.0[0]);
        }

        let m = self.0.len().max(other.0.len()) / 2;
        let mut half = m.min(self.0.len());

        let y = BigInt(self.0.range(0..half).copied().collect());
        let x = if half < self.0.len() {
            BigInt(self.0.range(half..).copied().collect())
        } else {
            BigInt::new()
        };

        half = m.min(other.0.len());

        let z = BigInt(other.0.range(0..half).copied().collect());
        let w = if half < other.0.len() {
            BigInt(other.0.range(half..).copied().collect())
        } else {
            BigInt::new()
        };

        let (mut xw, yz) = (&x * &w, &y * &z);
        let r = &(&x + &y) * &(&w + &z);
        let mut xz_plus_yw = &r - &(&xw + &yz);

        if !xw.is_zero() {
            for _ in 0..2 * m {
                xw.0.push_front(0);
            }
        }
        if !xz_plus_yw.is_zero() {
            for _ in 0..m {
                xz_plus_yw.0.push_front(0);
            }
        }

        &xw + &(&xz_plus_yw + &yz)
    }
}

impl fmt::Display for BigInt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.iter().rev().enumerate().for_each(|(i, num)| {
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
    let mut input = buf.lines().skip(1).map(BigInt::parse);

    let (a, b) = (input.next().unwrap(), input.next().unwrap());

    println!("{}", &a * &b);
}
