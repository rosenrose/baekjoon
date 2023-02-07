use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, Sub};

const DIGITS: usize = 37;
const EXP: i128 = 10_i128.pow(DIGITS as u32);

#[derive(PartialEq)]
struct BigInt {
    abs: Vec<i128>,
    sign: i8,
}

impl BigInt {
    fn from(abs: Vec<i128>, sign: i8) -> Self {
        Self { abs, sign }
    }

    fn parse(input: &str) -> Self {
        let mut sign = 1;
        let mut abs: Vec<_> = input
            .as_bytes()
            .rchunks(DIGITS)
            .map(|chunk| {
                let mut exp = 1;

                chunk.iter().rev().fold(0, |acc, &ch| {
                    if ch as char == '-' {
                        sign = -1;
                        return acc;
                    }

                    let num = (ch as i128 - '0' as i128) * exp;
                    exp *= 10;

                    acc + num
                })
            })
            .collect();

        if abs.len() > 1 && *abs.last().unwrap() == 0 {
            abs.pop();
        }

        Self { abs, sign }
    }

    fn len(&self) -> usize {
        self.abs.len()
    }

    fn abs(&self) -> Self {
        Self::from(self.abs.clone(), 1)
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
                let temp = carry + self.abs.get(i).unwrap_or(&0) + other.abs.get(i).unwrap_or(&0);
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
        let mut diff: Vec<_> = (0..self.len().max(other.len()))
            .map(|i| {
                let temp = carry + self.abs.get(i).unwrap_or(&0) - other.abs.get(i).unwrap_or(&0);

                if temp < 0 {
                    carry = -1;
                    temp + EXP
                } else {
                    carry = 0;
                    temp
                }
            })
            .collect();

        while diff.len() > 1 && *diff.last().unwrap() == 0 {
            diff.pop();
        }

        let mut result = Self::from(diff, self.sign);

        if result.len() == 1 && result.abs[0] == 0 {
            result.sign = 1;
        }

        result
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
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let mut input = buf.split_whitespace().map(BigInt::parse);
    let (a, b) = (input.next().unwrap(), input.next().unwrap());

    println!("{}", a + b);
}
