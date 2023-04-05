use std::cmp::Ordering;
use std::collections::VecDeque;
use std::fmt;
use std::io;
use std::ops::SubAssign;

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
    let mut input = buf.lines();
    let mut input = || input.next().unwrap();

    let n: i32 = input().parse().unwrap();

    for (a, b) in (0..n).map(|_| (input(), input())) {
        let (q, r) = BigInt::parse(a).divmod(BigInt::parse(b));

        println!("{q}\n{r}");
        println!("");
    }
}
