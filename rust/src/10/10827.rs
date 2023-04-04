use std::collections::VecDeque;
use std::fmt;
use std::ops::{Add, Mul};

#[derive(Clone)]
struct BigFloat {
    nums: VecDeque<i8>,
    exp: i32,
}

impl BigFloat {
    fn new() -> Self {
        Self::from(VecDeque::from([0]), 0)
    }

    fn from(nums: VecDeque<i8>, exp: i32) -> BigFloat {
        Self { nums, exp }
    }

    fn parse(input: &str) -> Self {
        let nums: VecDeque<_> = input
            .chars()
            .rev()
            .skip_while(|&ch| ch == '0')
            .filter_map(|ch| (ch != '.').then_some(ch as i8 - '0' as i8))
            .collect();

        let exp = if input.starts_with("0.") {
            let non_zero_idx = input.rfind(|ch| ch != '0').unwrap();
            (non_zero_idx - 1) as i32 * -1
        } else {
            if let Some((_, decimal)) = input.split_once('.') {
                decimal.trim_end_matches('0').len() as i32 * -1
            } else {
                input.chars().rev().take_while(|&ch| ch == '0').count() as i32
            }
        };

        let mut result = Self { nums, exp };

        result.zero_justify();
        result
    }

    fn is_zero(&self) -> bool {
        self.nums.iter().all(|&i| i == 0)
    }

    fn zero_justify(&mut self) {
        while self.nums.len() > 1 && self.nums.back() == Some(&0) {
            self.nums.pop_back();
        }

        while self.nums.len() > 1 && self.nums.front() == Some(&0) {
            self.nums.pop_front();
            self.exp += 1;
        }

        if self.is_zero() {
            self.exp = 0;
        }
    }

    fn mul_int(&self, other: i8) -> Self {
        let mut carry = 0;
        let mut result: VecDeque<_> = self
            .nums
            .iter()
            .map(|num| {
                let temp = carry + num * other;
                carry = temp / 10;

                temp % 10
            })
            .collect();

        if carry > 0 {
            result.push_back(carry);
        }

        Self::from(result, self.exp)
    }

    fn pow(&self, mut exp: i32) -> Self {
        let mut base = self.clone();
        let mut result = Self::from(VecDeque::from([1]), 0);

        loop {
            if exp % 2 == 1 {
                result = &result * &base;
            }

            if exp == 1 {
                break;
            }

            base = &base * &base;
            exp /= 2;
        }

        result.zero_justify();
        result
    }
}

impl Add for &BigFloat {
    type Output = BigFloat;

    fn add(self, other: Self) -> Self::Output {
        let mut carry = 0;
        let mut sum: VecDeque<_> = (0..self.nums.len().max(other.nums.len()))
            .map(|i| {
                let temp = carry + self.nums.get(i).unwrap_or(&0) + other.nums.get(i).unwrap_or(&0);
                carry = temp / 10;

                temp % 10
            })
            .collect();

        if carry > 0 {
            sum.push_back(carry);
        }

        BigFloat::from(sum, self.exp.min(other.exp))
    }
}

impl Mul for &BigFloat {
    type Output = BigFloat;

    fn mul(self, other: Self) -> Self::Output {
        if self.is_zero() || other.is_zero() {
            return BigFloat::new();
        }

        let mut result = other
            .nums
            .iter()
            .enumerate()
            .fold(BigFloat::new(), |acc, (i, &num)| {
                let mut result = self.mul_int(num);

                if !result.is_zero() {
                    for _ in 0..i {
                        result.nums.push_front(0);
                    }
                }

                &acc + &result
            });

        result.exp = self.exp + other.exp;
        result
    }
}

impl fmt::Display for BigFloat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_zero() {
            return write!(f, "0");
        }

        if self.exp >= 0 {
            self.nums.iter().rev().for_each(|num| {
                write!(f, "{num}").unwrap();
            });

            return write!(f, "{}", "0".repeat(self.exp as usize));
        }

        if self.exp.abs() as usize >= self.nums.len() {
            write!(f, "0.").unwrap();
            write!(
                f,
                "{}",
                "0".repeat(self.exp.abs() as usize - self.nums.len())
            )
            .unwrap();
        }

        self.nums.iter().enumerate().rev().for_each(|(i, num)| {
            write!(f, "{num}").unwrap();

            if i as i32 * -1 == self.exp {
                write!(f, ".").unwrap();
            }
        });

        Ok(())
    }
}

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let (a, b) = buf.trim().split_once(' ').unwrap();
    let (a, b) = (BigFloat::parse(a), b.parse::<i32>().unwrap());

    println!("{}", a.pow(b));
}

// assert_eq!(format!("{}", BigFloat::parse("0")), "0");
// assert_eq!(format!("{}", BigFloat::parse("0.0")), "0");
// assert_eq!(format!("{}", BigFloat::parse("0.000")), "0");
// assert_eq!(format!("{}", BigFloat::parse("000.0")), "0");

// assert_eq!(format!("{}", BigFloat::parse("1")), "1");
// assert_eq!(format!("{}", BigFloat::parse("1.0")), "1");
// assert_eq!(format!("{}", BigFloat::parse("1.00")), "1");
// assert_eq!(format!("{}", BigFloat::parse("01")), "1");
// assert_eq!(format!("{}", BigFloat::parse("01.0")), "1");
// assert_eq!(format!("{}", BigFloat::parse("001.0")), "1");

// assert_eq!(format!("{}", BigFloat::parse("0.0012")), "0.0012");
// assert_eq!(format!("{}", BigFloat::parse("0.00120")), "0.0012");
// assert_eq!(format!("{}", BigFloat::parse("0.001200")), "0.0012");
// assert_eq!(format!("{}", BigFloat::parse("00.012")), "0.012");
// assert_eq!(format!("{}", BigFloat::parse("0.12")), "0.12");
// assert_eq!(format!("{}", BigFloat::parse("1.2")), "1.2");
// assert_eq!(format!("{}", BigFloat::parse("12")), "12");
// assert_eq!(format!("{}", BigFloat::parse("120")), "120");
// assert_eq!(format!("{}", BigFloat::parse("1200")), "1200");

// assert_eq!(format!("{}", BigFloat::parse("0.002103")), "0.002103");
// assert_eq!(format!("{}", BigFloat::parse("0.02103")), "0.02103");
// assert_eq!(format!("{}", BigFloat::parse("0.2103")), "0.2103");
// assert_eq!(format!("{}", BigFloat::parse("2.103")), "2.103");
// assert_eq!(format!("{}", BigFloat::parse("21.03")), "21.03");
// assert_eq!(format!("{}", BigFloat::parse("210.3")), "210.3");
// assert_eq!(format!("{}", BigFloat::parse("2103.0")), "2103");
// assert_eq!(format!("{}", BigFloat::parse("2103")), "2103");
// assert_eq!(format!("{}", BigFloat::parse("21030")), "21030");
// assert_eq!(format!("{}", BigFloat::parse("210300")), "210300");
