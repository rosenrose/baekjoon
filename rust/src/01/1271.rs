use std::cmp::Ordering;
use std::collections::VecDeque;
use std::fmt;
use std::ops::SubAssign;

#[derive(PartialEq, Debug)]
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

    fn divmod(&self, other: Self) -> (Self, Self) {
        let (mut dividend, mut quotient) = (Self::new(), Self::new());

        for &num in self.0.iter().rev() {
            let mut q = 0;

            dividend.0.push_front(num);
            dividend.zero_justify();

            while dividend >= other {
                dividend -= &other;
                q += 1;
            }

            quotient.0.push_front(q);
        }

        quotient.zero_justify();

        (quotient, dividend)
    }
}

impl SubAssign<&BigInt> for BigInt {
    fn sub_assign(&mut self, other: &Self) {
        let mut carry = 0;

        for i in 0..self.0.len() {
            let temp = carry + self.0[i] - other.0.get(i).unwrap_or(&0);

            if temp < 0 {
                carry = -1;
                self.0[i] = temp + 10;
            } else {
                carry = 0;
                self.0[i] = temp;
            }
        }

        self.zero_justify();
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

        Ok(())
    }
}

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let (n, m) = buf.trim().split_once(' ').unwrap();
    let (q, r) = BigInt::parse(n).divmod(BigInt::parse(m));

    println!("{q}\n{r}");
}
