use std::cmp::Ordering;
use std::collections::VecDeque;
use std::fmt;
use std::io;
use std::ops::{Add, Div, Sub};

#[derive(PartialEq)]
struct BigInt(VecDeque<i8>);

impl BigInt {
    fn new() -> Self {
        Self(VecDeque::from([0]))
    }

    fn parse(input: &str) -> Self {
        Self(input.chars().rev().map(|c| c as i8 - '0' as i8).collect())
    }

    fn zero_justify(&mut self) {
        while self.0.len() > 1 && self.0.back() == Some(&0) {
            self.0.pop_back();
        }
    }
}

impl Add for &BigInt {
    type Output = BigInt;

    fn add(self, other: Self) -> Self::Output {
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
                    temp + 10
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
impl Div for &BigInt {
    type Output = BigInt;

    fn div(self, other: Self) -> Self::Output {
        let (mut dividend, mut quotient) = (BigInt::new(), BigInt::new());

        for &num in self.0.iter().rev() {
            let mut q = 0;

            dividend.0.push_front(num);
            dividend.zero_justify();

            while dividend >= *other {
                dividend = &dividend - &other;
                q += 1;
            }

            quotient.0.push_front(q);
        }

        quotient.zero_justify();
        quotient
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
        self.0.iter().rev().for_each(|num| {
            write!(f, "{num}").unwrap();
        });

        Ok(())
    }
}

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines().map(BigInt::parse);

    let (n, x) = (input.next().unwrap(), input.next().unwrap());
    let b = &(&n - &x) / &BigInt(VecDeque::from([2]));
    let a = &b + &x;

    println!("{a}\n{b}");
}
