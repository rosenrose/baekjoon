use std::cmp::Ordering;
use std::collections::VecDeque;
use std::fmt;

#[derive(PartialEq, Debug)]
struct BigInt {
    abs: VecDeque<i8>,
    sign: i8,
}

impl BigInt {
    fn new() -> Self {
        Self {
            abs: VecDeque::new(),
            sign: 1,
        }
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

    fn add(&self, other: i8) -> Self {
        let mut carry = 0;
        let mut result = self.abs.clone();
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
            abs: result,
            sign: self.sign,
        }
    }

    fn sub(&self, other: &Self) -> Self {
        let mut carry = 0;
        let mut diff: VecDeque<_> = (0..self.abs.len().max(other.abs.len()))
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

        Self { abs: diff, sign: 1 }
    }

    fn push_front(&mut self, num: i8) {
        if self.is_zero() {
            self.abs.clear();
        }

        self.abs.push_front(num);
    }

    fn divmod(&self, other: Self) -> (Self, Self) {
        let (mut dividend, mut quotient) = (Self::new(), Self::new());

        for &num in self.abs.iter().rev() {
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

    fn is_zero(&self) -> bool {
        self.abs.iter().all(|&i| i == 0)
    }
}

impl PartialOrd for BigInt {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.abs.len() == other.abs.len() {
            Some(self.abs.iter().rev().cmp(other.abs.iter().rev()))
        } else {
            Some(self.abs.len().cmp(&other.abs.len()))
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
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let mut input = buf.split_whitespace().map(BigInt::parse);
    let (a, b) = (input.next().unwrap(), input.next().unwrap());
    let (q, r) = a.divmod(b);

    println!("{q}\n{r}");
}
