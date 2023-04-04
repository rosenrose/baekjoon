use std::cmp::Ordering;
use std::collections::VecDeque;
use std::fmt;
use std::io;
use std::ops::{Add, Sub};

#[derive(Clone, PartialEq, Debug)]
struct BigFloat {
    nums: VecDeque<i8>,
    point: i8,
    sign: i8,
}

impl BigFloat {
    fn new() -> Self {
        Self::from(VecDeque::from([0]), 0, 1)
    }

    fn from(nums: VecDeque<i8>, point: i8, sign: i8) -> BigFloat {
        Self { nums, point, sign }
    }

    fn parse(input: &str) -> Self {
        let sign = if input.starts_with('-') { -1 } else { 1 };
        let input = input.trim_start_matches('-');

        let nums: VecDeque<_> = input
            .chars()
            .rev()
            .filter_map(|ch| (ch != '.').then_some(ch as i8 - '0' as i8))
            .collect();

        let point = if input.starts_with("0.") {
            1
        } else {
            input.find('.').unwrap_or(input.len()) as i8
        };

        let mut result = Self { nums, point, sign };

        result.zero_justify();
        result
    }

    fn is_zero(&self) -> bool {
        self.nums.iter().all(|&i| i == 0)
    }

    fn zero_justify(&mut self) {
        while self.nums.len() > 1 && self.nums.back() == Some(&0) {
            self.nums.pop_back();
            self.point -= 1;
        }

        if self.point <= 0 {
            while self.nums.len() > 1 && self.nums.front() == Some(&0) {
                self.nums.pop_front();
            }
        } else {
            while self.nums.len() > self.point as usize && self.nums.front() == Some(&0) {
                self.nums.pop_front();
            }
        }

        if self.is_zero() {
            self.point = 0;
            self.sign = 1;
        }
    }

    fn abs(&self) -> Self {
        Self::from(self.nums.clone(), self.point, 1)
    }
}

impl Add for &BigFloat {
    type Output = BigFloat;

    fn add(self, other: Self) -> Self::Output {
        // println!("add: {self} {other}");
        if self.sign * other.sign == -1 {
            return if self.sign == 1 {
                self - &other.abs()
            } else {
                other - &self.abs()
            };
        }

        let (mut small, mut big) = if self.point < other.point {
            (self.clone(), other.clone())
        } else {
            (other.clone(), self.clone())
        };

        let small_len = small.point.abs_diff(big.point) as usize + small.nums.len();
        let len_diff = big.nums.len().abs_diff(small_len);

        if big.nums.len() < small_len {
            for _ in 0..len_diff {
                big.nums.push_front(0);
            }
        } else {
            for _ in 0..len_diff {
                small.nums.push_front(0);
            }
        }

        let mut point = big.point;
        let mut carry = 0;
        let mut sum: VecDeque<_> = (0..big.nums.len())
            .map(|i| {
                let temp = carry + big.nums[i] + small.nums.get(i).unwrap_or(&0);
                carry = temp / 10;

                temp % 10
            })
            .collect();

        if carry > 0 {
            sum.push_back(carry);
            point += 1;
        }

        let mut result = BigFloat::from(sum, point, self.sign);

        result.zero_justify();
        result
    }
}
impl Sub for &BigFloat {
    type Output = BigFloat;

    fn sub(self, other: Self) -> Self::Output {
        // println!("sub: {self} {other}");
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

        let (mut big, mut small) = (self.clone(), other.clone());
        let small_len = small.point.abs_diff(big.point) as usize + small.nums.len();
        let len_diff = big.nums.len().abs_diff(small_len);

        if big.nums.len() < small_len {
            for _ in 0..len_diff {
                big.nums.push_front(0);
            }
        } else {
            for _ in 0..len_diff {
                small.nums.push_front(0);
            }
        }

        let mut carry = 0;
        let diff: VecDeque<_> = (0..big.nums.len())
            .map(|i| {
                let temp = carry + big.nums[i] - small.nums.get(i).unwrap_or(&0);

                if temp < 0 {
                    carry = -1;
                    temp + 10
                } else {
                    carry = 0;
                    temp
                }
            })
            .collect();

        let mut result = BigFloat::from(diff, big.point, self.sign);

        result.zero_justify();
        result
    }
}

impl PartialOrd for BigFloat {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.is_zero() {
            return Some(Ordering::Less);
        }
        if other.is_zero() {
            return Some(Ordering::Greater);
        }

        if self.point != other.point {
            return Some(self.point.cmp(&other.point));
        }

        Some(
            format!("{self}")
                .replace(['-', '.'], "")
                .cmp(&format!("{other}").replace(['-', '.'], "")),
        )
    }
}

impl fmt::Display for BigFloat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_zero() {
            return write!(f, "0");
        }

        if self.sign == -1 {
            write!(f, "-").unwrap();
        }

        if self.point <= 0 {
            write!(f, "0.").unwrap();
            write!(f, "{}", "0".repeat(self.point.abs() as usize)).unwrap();
        }

        self.nums.iter().rev().enumerate().for_each(|(i, num)| {
            if self.point > 0 && i as i8 == self.point {
                write!(f, ".").unwrap();
            }

            write!(f, "{num}").unwrap();
        });

        Ok(())
    }
}

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines();

    let n: i32 = input.next().unwrap().parse().unwrap();

    for _ in 0..n {
        let sum = input
            .by_ref()
            .take_while(|&input| input != "0")
            .fold(BigFloat::new(), |acc, num| &acc + &BigFloat::parse(num));

        println!("{sum}");
    }
}
