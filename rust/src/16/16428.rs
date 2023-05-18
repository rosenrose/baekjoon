use std::cmp::Ordering;
use std::collections::VecDeque;
use std::fmt;
use std::ops::Sub;

#[derive(PartialEq, Debug)]
struct BigInt {
    nums: VecDeque<i8>,
    sign: i8,
}

impl BigInt {
    fn new() -> Self {
        Self::from(VecDeque::from([0]), 1)
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

    fn add(&self, other: i8) -> Self {
        let mut carry = 0;
        let mut result = self.nums.clone();
        result[0] += other;

        for num in result.iter_mut() {
            let temp = carry + *num;

            carry = temp / 10;
            *num = temp % 10;

            if carry == 0 {
                break;
            }
        }

        if carry > 0 {
            result.push_back(carry);
        }

        Self::from(result, self.sign)
    }

    fn divmod(&self, other: Self) -> (Self, Self) {
        let (mut dividend, mut quotient) = (Self::new(), Self::new());

        for &num in self.nums.iter().rev() {
            let mut q = 0;

            dividend.nums.push_front(num);
            dividend.zero_justify();

            while dividend >= other {
                dividend = &dividend - &other;
                q += 1;
            }

            quotient.nums.push_front(q);
        }

        let mut remainder = dividend;

        if (self.sign, other.sign) == (1, -1) {
            quotient.sign = -1;
        }

        if self.sign == -1 {
            if other.sign == 1 {
                quotient.sign = -1;
            }

            if !remainder.is_zero() {
                quotient = quotient.add(1);
                remainder = &other - &remainder;
            }
        }

        quotient.zero_justify();

        (quotient, remainder)
    }
}

impl Sub for &BigInt {
    type Output = BigInt;

    fn sub(self, other: Self) -> Self::Output {
        let mut carry = 0;
        let diff: VecDeque<_> = (0..self.nums.len().max(other.nums.len()))
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

        let mut result = BigInt::from(diff, 1);

        result.zero_justify();
        result
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

        self.nums.iter().rev().for_each(|num| {
            write!(f, "{num}").unwrap();
        });

        Ok(())
    }
}

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let (a, b) = buf.trim().split_once(' ').unwrap();
    let (q, r) = BigInt::parse(a).divmod(BigInt::parse(b));

    println!("{q}\n{r}");
}
