use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, Sub};

const DIGITS: usize = 37;
const EXP: i128 = 10_i128.pow(DIGITS as u32);

#[derive(PartialEq)]
struct BigInt {
    nums: Vec<i128>,
    sign: i8,
}

impl BigInt {
    fn from(nums: Vec<i128>, sign: i8) -> Self {
        Self { nums, sign }
    }

    fn parse(input: &str) -> Self {
        let mut result = Self {
            nums: input
                .as_bytes()
                .rchunks(DIGITS)
                .map(|chunk| {
                    let mut exp = 1;

                    chunk.iter().rev().fold(0, |acc, &ch| {
                        if ch as char == '-' {
                            return acc;
                        }

                        let num = (ch as i128 - '0' as i128) * exp;
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
        self.nums.iter().all(|&i| i == 0)
    }

    fn zero_justify(&mut self) {
        while self.nums.len() > 1 && self.nums.last() == Some(&0) {
            self.nums.pop();
        }

        if self.is_zero() {
            self.sign = 1;
        }
    }

    fn len(&self) -> usize {
        self.nums.len()
    }

    fn abs(&self) -> Self {
        Self::from(self.nums.clone(), 1)
    }
}

impl Add for BigInt {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        if self.sign * other.sign == -1 {
            return if self.sign == 1 {
                self - other.abs()
            } else {
                other - self.abs()
            };
        }

        let mut carry = 0;
        let mut sum: Vec<_> = (0..self.len().max(other.len()))
            .map(|i| {
                let temp = carry + self.nums.get(i).unwrap_or(&0) + other.nums.get(i).unwrap_or(&0);
                carry = temp / EXP;

                temp % EXP
            })
            .collect();

        if carry > 0 {
            sum.push(carry);
        }

        Self::from(sum, self.sign)
    }
}
impl Sub for BigInt {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        if self.sign * other.sign == -1 {
            return if self.sign == 1 {
                self + other.abs()
            } else {
                let mut result = other + self.abs();
                result.sign = -1;

                result
            };
        }

        if self < other {
            let mut result = other - self;
            result.sign *= -1;

            return result;
        }

        let mut carry = 0;
        let diff: Vec<_> = (0..self.len().max(other.len()))
            .map(|i| {
                let temp = carry + self.nums.get(i).unwrap_or(&0) - other.nums.get(i).unwrap_or(&0);

                if temp < 0 {
                    carry = -1;
                    temp + EXP
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
}

impl PartialOrd for BigInt {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.len() == other.len() {
            Some(self.nums.iter().rev().cmp(other.nums.iter().rev()))
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

        self.nums.iter().rev().enumerate().for_each(|(i, num)| {
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
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let (a, b) = buf.trim().split_once(' ').unwrap();

    println!("{}", BigInt::parse(a) + BigInt::parse(b));
}
