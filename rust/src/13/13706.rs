use std::cmp::Ordering;
use std::collections::VecDeque;
use std::fmt;
use std::ops::{Add, Mul, Sub};

const DIGITS: usize = 18;
const POW: i128 = 10_i128.pow(DIGITS as u32);

#[derive(PartialEq, Clone)]
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
                    chunk
                        .iter()
                        .fold(0, |acc, ch| acc * 10 + (ch - b'0') as i64)
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

    fn len(&self) -> usize {
        self.0.len()
    }

    fn mul_int(&self, other: i64) -> Self {
        let mut carry = 0;
        let mut result: VecDeque<_> = self
            .0
            .iter()
            .map(|&num| {
                let temp = carry + num as i128 * other.abs() as i128;
                carry = temp / POW;

                (temp % POW) as i64
            })
            .collect();

        if carry > 0 {
            result.push_back(carry as i64);
        }

        Self(result)
    }

    fn div_two(&self) -> Self {
        let mut carry = 0;
        let mut result = BigInt::new();

        for num in self.0.iter().rev() {
            let temp = carry + num;
            carry = (temp & 1) * POW as i64;

            result.0.push_front(temp >> 1);
        }

        result.zero_justify();
        result
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
                carry = temp / POW;

                (temp % POW) as i64
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
                    temp + POW as i64
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
        if self.len() == 1 {
            return other.mul_int(self.0[0]);
        }
        if other.len() == 1 {
            return self.mul_int(other.0[0]);
        }

        let m = self.len().max(other.len()) / 2;
        let mut half = m.min(self.len());

        let y = BigInt(self.0.range(0..half).copied().collect());
        let x = if half < self.len() {
            BigInt(self.0.range(half..).copied().collect())
        } else {
            BigInt::new()
        };

        half = m.min(other.len());

        let z = BigInt(other.0.range(0..half).copied().collect());
        let w = if half < other.len() {
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

impl PartialOrd for BigInt {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(
            self.0
                .len()
                .cmp(&other.0.len())
                .then_with(|| self.0.iter().rev().cmp(other.0.iter().rev())),
        )
    }
}

impl fmt::Display for BigInt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, num) in self.0.iter().rev().enumerate() {
            if i == 0 {
                write!(f, "{num}")
            } else {
                write!(f, "{num:0DIGITS$}")
            }
            .unwrap()
        }

        Ok(())
    }
}

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let n = BigInt::parse(buf.trim());

    println!("{}", binary_search(n));
}

fn binary_search(n: BigInt) -> BigInt {
    let is_ok = |num: &BigInt| num * num <= n;
    let one = BigInt(VecDeque::from([1]));

    let half = n.len() / 2;
    let (mut lo, mut hi) = (
        one.clone(),
        BigInt((0..half + 1).map(|_| POW as i64 - 1).collect()),
    );
    let mut result = BigInt::new();

    while lo <= hi {
        let mid = (&lo + &hi).div_two();

        if is_ok(&mid) {
            lo = &mid + &one;
            result = mid;
        } else {
            hi = &mid - &one;
        }
    }

    result
}
