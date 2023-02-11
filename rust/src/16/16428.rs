use std::cmp::Ordering;
use std::collections::VecDeque;
use std::fmt;

#[derive(PartialEq, Debug)]
struct BigInt {
    nums: VecDeque<i8>,
    sign: i8,
}

impl BigInt {
    fn new() -> Self {
        Self {
            nums: VecDeque::new(),
            sign: 1,
        }
    }

    fn parse(input: &str) -> Self {
        Self {
            nums: input
                .chars()
                .rev()
                .filter_map(|c| (c != '-').then_some(c as i8 - '0' as i8))
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

        Self {
            nums: result,
            sign: self.sign,
        }
    }

    fn sub(&self, other: &Self) -> Self {
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

        let mut result = Self {
            nums: diff,
            sign: 1,
        };

        result.zero_justify();
        result
    }

    fn push_front(&mut self, num: i8) {
        if self.is_zero() {
            self.nums.clear();
        }

        self.nums.push_front(num);
    }

    fn divmod(&self, other: Self) -> (Self, Self) {
        let (mut dividend, mut quotient) = (Self::new(), Self::new());

        for &num in self.nums.iter().rev() {
            let mut q = 0;

            dividend.push_front(num);

            while dividend >= other {
                dividend = dividend.sub(&other);
                q += 1;
            }

            quotient.push_front(q);
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
                remainder = other.sub(&remainder);
            }
        }

        if quotient.is_zero() {
            quotient.sign = 1;
        }

        (quotient, remainder)
    }
}

impl PartialOrd for BigInt {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.nums.len() == other.nums.len() {
            Some(self.nums.iter().rev().cmp(other.nums.iter().rev()))
        } else {
            Some(self.nums.len().cmp(&other.nums.len()))
        }
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

        write!(f, "")
    }
}

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let mut input = buf.split_whitespace().map(BigInt::parse);
    let (a, b) = (input.next().unwrap(), input.next().unwrap());
    let (q, r) = a.divmod(b);

    println!("{q}\n{r}");
}
