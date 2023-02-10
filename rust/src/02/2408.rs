use std::cmp::Ordering;
use std::collections::VecDeque;
use std::fmt;
use std::io;
use std::ops::{Add, AddAssign, Div, Mul, Sub, SubAssign};

#[derive(Clone, Default, PartialEq)]
struct BigInt {
    abs: VecDeque<i8>,
    sign: i8,
}

impl BigInt {
    fn new() -> Self {
        Self {
            abs: VecDeque::from([0]),
            sign: 1,
        }
    }

    fn from(abs: VecDeque<i8>, sign: i8) -> Self {
        Self { abs, sign }
    }

    fn parse(input: &str) -> Self {
        Self {
            abs: input
                .chars()
                .rev()
                .filter_map(|c| (c != '-').then_some(c as i8 - '0' as i8))
                .collect(),
            sign: if input.starts_with('-') { -1 } else { 1 },
        }
    }

    fn mul_int(&self, other: i8) -> Self {
        let mut carry = 0;
        let mut result: VecDeque<_> = self
            .abs
            .iter()
            .map(|&num| {
                let temp = carry + num * other.abs();
                carry = temp / 10;

                temp % 10
            })
            .collect();

        if carry > 0 {
            result.push_back(carry);
        }

        Self::from(result, self.sign * other.signum() as i8)
    }

    fn is_zero(&self) -> bool {
        self.abs.iter().all(|&i| i == 0)
    }

    fn len(&self) -> usize {
        self.abs.len()
    }

    fn abs(&self) -> Self {
        Self::from(self.abs.clone(), 1)
    }

    fn push_front(&mut self, num: i8) {
        if self.is_zero() {
            self.abs.clear();
        }

        self.abs.push_front(num);
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
        let mut sum: VecDeque<_> = (0..self.len().max(other.len()))
            .map(|i| {
                let temp = carry + self.abs.get(i).unwrap_or(&0) + other.abs.get(i).unwrap_or(&0);
                carry = temp / 10;

                temp % 10
            })
            .collect();

        if carry > 0 {
            sum.push_back(carry);
        }

        Self::from(sum, self.sign)
    }
}
impl AddAssign for BigInt {
    fn add_assign(&mut self, other: Self) {
        *self = (*self).clone() + other;
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
        let mut diff: VecDeque<_> = (0..self.len().max(other.len()))
            .map(|i| {
                let temp = carry + self.abs.get(i).unwrap_or(&0) - other.abs.get(i).unwrap_or(&0);

                if temp < 0 {
                    carry = -1;
                    temp + 10
                } else {
                    carry = 0;
                    temp
                }
            })
            .collect();

        while diff.len() > 1 && diff.back() == Some(&0) {
            diff.pop_back();
        }

        let mut result = Self::from(diff, self.sign);

        if result.is_zero() {
            result.sign = 1;
        }

        result
    }
}
impl SubAssign for BigInt {
    fn sub_assign(&mut self, other: Self) {
        *self = (*self).clone() - other;
    }
}
impl Mul for BigInt {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        if self.is_zero() || other.is_zero() {
            return Self::new();
        }
        if self.len() == 1 {
            return other.mul_int(self.abs[0] * self.sign);
        }
        if other.len() == 1 {
            return self.mul_int(other.abs[0] * other.sign);
        }

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

        let (mut xw, yz) = (x.clone() * w.clone(), y.clone() * z.clone());
        let r = (x + y) * (w + z);
        let mut xz_plus_yw = r - (xw.clone() + yz.clone());

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

        Self::from((xw + xz_plus_yw + yz).abs, self.sign * other.sign)
    }
}
impl Div for BigInt {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        let (mut dividend, mut quotient) = (Self::new(), Self::new());

        for &num in self.abs.iter().rev() {
            let mut q = 0;

            dividend.push_front(num);

            while dividend >= other {
                dividend -= other.abs();
                q += 1;
            }

            quotient.push_front(q);
        }

        let remainder = dividend;

        if self.sign * other.sign == -1 {
            if !remainder.is_zero() {
                quotient += BigInt::from(VecDeque::from([1]), 1);
            }

            quotient.sign = -1;
        }

        if quotient.is_zero() {
            quotient.sign = 1;
        }

        quotient
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

        self.abs.iter().rev().for_each(|num| {
            write!(f, "{num}").unwrap();
        });

        write!(f, "")
    }
}

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();

    let mut stack = Vec::new();
    let mut postfix = Vec::new();

    for input in buf.lines().skip(1) {
        match input {
            "+" | "-" | "*" | "/" => {
                while let Some(&token) = stack.last() {
                    if precedence(token) < precedence(input) {
                        break;
                    }

                    postfix.push(stack.pop().unwrap());
                }

                stack.push(input);
            }
            _ => postfix.push(input),
        };
    }

    while let Some(token) = stack.pop() {
        postfix.push(token);
    }
    // println!("{postfix:?}");
    let mut stack = Vec::<BigInt>::new();

    for token in postfix {
        if !matches!(token, "+" | "-" | "*" | "/") {
            stack.push(BigInt::parse(token));
            continue;
        }

        let (b, a) = (stack.pop().unwrap(), stack.pop().unwrap());
        let result = match token {
            "+" => a + b,
            "-" => a - b,
            "*" => a * b,
            "/" => a / b,
            _ => Default::default(),
        };

        stack.push(result);
    }

    println!("{}", stack.pop().unwrap());
}

fn precedence(op: &str) -> i32 {
    match op {
        "+" | "-" => 1,
        "*" | "/" => 2,
        _ => Default::default(),
    }
}
