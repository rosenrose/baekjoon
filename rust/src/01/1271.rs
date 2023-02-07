use std::cmp::Ordering;
use std::collections::VecDeque;
use std::fmt;
use std::ops::{Add, Sub, SubAssign};

#[derive(Clone, Debug, PartialEq)]
struct BigInt(VecDeque<i8>);

impl BigInt {
    fn new() -> Self {
        Self(VecDeque::new())
    }

    fn parse(input: &str) -> Self {
        Self(input.chars().rev().map(|c| c as i8 - '0' as i8).collect())
    }

    fn mul(&self, other: i8) -> Self {
        let mut carry = 0;
        let mut result: VecDeque<_> = self
            .0
            .iter()
            .map(|&num| {
                let temp = carry + num * other;
                carry = temp / 10;

                temp % 10
            })
            .collect();

        if carry > 0 {
            result.push_back(carry);
        }

        Self(result)
    }

    fn divmod(&self, other: Self) -> (Self, Self) {
        let (mut dividend, mut quotient) = (Self::new(), Self::new());

        for i in (0..=self.0.len() - other.0.len()).rev() {
            if dividend.0.is_empty() {
                dividend = Self(self.0.range(i..).copied().collect());
            } else {
                if dividend.is_zero() {
                    dividend.0.clear();
                }

                dividend.0.push_front(self.0[i]);
            }
            // println!("{dividend:?} {quotient:?}");
            if dividend < other {
                if i < self.0.len() - other.0.len() {
                    quotient.0.push_front(0);
                }
                continue;
            }

            let mut q = 9;
            let mut divisor = other.mul(q);

            while divisor > dividend {
                q -= 1;
                divisor -= other.clone();
            }

            quotient.0.push_front(q);
            dividend -= divisor;
        }

        (quotient, dividend)
    }

    fn is_zero(&self) -> bool {
        self.0.iter().all(|&i| i == 0)
    }
}

impl Add for BigInt {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut carry = 0;
        let mut sum: VecDeque<_> = (0..self.0.len().max(other.0.len()))
            .map(|i| {
                let temp = carry + self.0.get(i).unwrap_or(&0) + other.0.get(i).unwrap_or(&0);
                carry = temp / 10;

                temp % 10
            })
            .collect();

        if carry > 0 {
            sum.push_back(carry);
        }

        Self(sum)
    }
}
impl Sub for BigInt {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let mut carry = 0;
        let mut diff: VecDeque<_> = (0..self.0.len().max(other.0.len()))
            .map(|i| {
                let temp = carry + self.0.get(i).unwrap_or(&0) - other.0.get(i).unwrap_or(&0);

                if temp < 0 {
                    carry = -1;
                    temp + 10
                } else {
                    carry = 0;
                    temp
                }
            })
            .collect();

        while diff.len() > 1 && *diff.back().unwrap() == 0 {
            diff.pop_back();
        }

        Self(diff)
    }
}
impl SubAssign for BigInt {
    fn sub_assign(&mut self, other: Self) {
        *self = (*self).clone() - other;
    }
}

impl PartialOrd for BigInt {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.0.len() == other.0.len() {
            Some(self.0.iter().rev().cmp(other.0.iter().rev()))
        } else {
            Some(self.0.len().cmp(&other.0.len()))
        }
    }
}

impl fmt::Display for BigInt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.iter().rev().for_each(|num| {
            write!(f, "{num}").unwrap();
        });

        write!(f, "")
    }
}

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let mut input = buf.split_whitespace().map(BigInt::parse);
    let (n, m) = (input.next().unwrap(), input.next().unwrap());
    let (q, r) = n.divmod(m);

    println!("{q}\n{r}");
}
