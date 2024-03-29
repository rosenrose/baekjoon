use std::cmp::Ordering;
use std::collections::VecDeque;
use std::fmt;
use std::io;
use std::ops::{Add, Div, Mul, Sub};

#[derive(PartialEq)]
struct BigInt {
    nums: VecDeque<i8>,
    sign: i8,
}

impl BigInt {
    fn new() -> Self {
        Self {
            nums: VecDeque::from([0]),
            sign: 1,
        }
    }

    fn from(nums: VecDeque<i8>, sign: i8) -> Self {
        Self { nums, sign }
    }

    fn parse(input: &str) -> Self {
        Self {
            nums: input
                .as_bytes()
                .iter()
                .rev()
                .filter_map(|&ch| (ch != b'-').then_some((ch - b'0') as i8))
                .collect(),
            sign: if input.starts_with('-') { -1 } else { 1 },
        }
    }

    fn is_zero(&self) -> bool {
        self.nums.iter().all(|&i| i == 0)
    }

    fn zero_justify(&mut self) {
        while self.nums.len() > 1 && self.nums.back() == Some(&0) {
            self.nums.pop_back();
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

    fn mul_int(&self, other: i8) -> Self {
        let mut carry = 0;
        let mut result: VecDeque<_> = self
            .nums
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
}

impl Add for &BigInt {
    type Output = BigInt;

    fn add(self, other: Self) -> Self::Output {
        if self.sign * other.sign == -1 {
            return if self.sign == 1 {
                self - &other.abs()
            } else {
                other - &self.abs()
            };
        }

        let mut carry = 0;
        let mut sum: VecDeque<_> = (0..self.len().max(other.len()))
            .map(|i| {
                let temp = carry + self.nums.get(i).unwrap_or(&0) + other.nums.get(i).unwrap_or(&0);
                carry = temp / 10;

                temp % 10
            })
            .collect();

        if carry > 0 {
            sum.push_back(carry);
        }

        BigInt::from(sum, self.sign)
    }
}
impl Sub for &BigInt {
    type Output = BigInt;

    fn sub(self, other: Self) -> Self::Output {
        if self.sign * other.sign == -1 {
            return if self.sign == 1 {
                self + &other.abs()
            } else {
                let mut result = other + &self.abs();
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
        let diff: VecDeque<_> = (0..self.len().max(other.len()))
            .map(|i| {
                let temp = carry + self.nums.get(i).unwrap_or(&0) - other.nums.get(i).unwrap_or(&0);

                if temp < 0 {
                    carry = -1;
                    temp + 10
                } else {
                    carry = 0;
                    temp
                }
            })
            .collect();

        let mut result = BigInt::from(diff, self.sign);

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
            return other.mul_int(self.nums[0] * self.sign);
        }
        if other.len() == 1 {
            return self.mul_int(other.nums[0] * other.sign);
        }

        let m = self.len().max(other.len()) / 2;
        let mut half = m.min(self.len());

        let y = BigInt::from(self.nums.range(0..half).copied().collect(), 1);
        let x = if half < self.len() {
            BigInt::from(self.nums.range(half..).copied().collect(), 1)
        } else {
            BigInt::new()
        };

        half = m.min(other.len());

        let z = BigInt::from(other.nums.range(0..half).copied().collect(), 1);
        let w = if half < other.len() {
            BigInt::from(other.nums.range(half..).copied().collect(), 1)
        } else {
            BigInt::new()
        };

        let (mut xw, yz) = (&x * &w, &y * &z);
        let r = &(&x + &y) * &(&w + &z);
        let mut xz_plus_yw = &r - &(&xw + &yz);

        if !xw.is_zero() {
            for _ in 0..2 * m {
                xw.nums.push_front(0);
            }
        }
        if !xz_plus_yw.is_zero() {
            for _ in 0..m {
                xz_plus_yw.nums.push_front(0);
            }
        }

        BigInt::from((&xw + &(&xz_plus_yw + &yz)).nums, self.sign * other.sign)
    }
}
impl Div for &BigInt {
    type Output = BigInt;

    fn div(self, other: Self) -> Self::Output {
        let (mut dividend, mut quotient) = (BigInt::new(), BigInt::new());

        for &num in self.nums.iter().rev() {
            let mut q = 0;

            dividend.nums.push_front(num);
            dividend.zero_justify();

            while dividend >= *other {
                dividend = &dividend - &other.abs();
                q += 1;
            }

            quotient.nums.push_front(q);
        }

        let remainder = dividend;

        if self.sign * other.sign == -1 {
            if !remainder.is_zero() {
                quotient = &quotient + &BigInt::from(VecDeque::from([1]), 1);
            }

            quotient.sign = -1;
        }

        quotient.zero_justify();
        quotient
    }
}

impl PartialOrd for BigInt {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(
            self.nums
                .len()
                .cmp(&other.nums.len())
                .then_with(|| self.nums.iter().rev().cmp(other.nums.iter().rev())),
        )
    }
}

impl fmt::Display for BigInt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.sign == -1 {
            write!(f, "-").unwrap();
        }

        for num in self.nums.iter().rev() {
            write!(f, "{num}").unwrap();
        }

        Ok(())
    }
}

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();

    let infix: Vec<_> = buf.lines().skip(1).collect();
    let postfix = infix_to_postfix(infix);
    // println!("{postfix:?}");
    let mut stack = Vec::new();

    for token in postfix {
        if !matches!(token, "+" | "-" | "*" | "/") {
            stack.push(BigInt::parse(token));
            continue;
        }

        let (b, a) = (stack.pop().unwrap(), stack.pop().unwrap());
        let result = match token {
            "+" => &a + &b,
            "-" => &a - &b,
            "*" => &a * &b,
            "/" => &a / &b,
            _ => unreachable!(),
        };

        stack.push(result);
    }

    println!("{}", stack.pop().unwrap());
}

fn infix_to_postfix(infix: Vec<&str>) -> Vec<&str> {
    let mut stack = Vec::new();
    let mut postfix = Vec::new();
    let precedence = |op: &str| match op {
        "+" | "-" => 1,
        "*" | "/" => 2,
        _ => 0,
    };

    for input in infix {
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

    postfix
}
