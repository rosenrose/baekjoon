use std::cmp::Ordering;
use std::io;
use std::ops::{Add, AddAssign};

const DIGITS: usize = 18;
const EXP: i128 = 10_i128.pow(DIGITS as u32);

#[derive(Clone, Eq, PartialEq, Debug)]
struct BigInt(Vec<i128>);

impl BigInt {
    fn mul(&self, other: i128) -> Self {
        let mut carry = 0;
        let mut result: Vec<_> = self
            .0
            .iter()
            .map(|&num| {
                let temp = carry + num * other;
                carry = temp / EXP;

                temp % EXP
            })
            .collect();

        if carry > 0 {
            result.push(carry);
        }

        Self(result)
    }
}

impl Add for BigInt {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut carry = 0;
        let mut sum: Vec<_> = (0..self.0.len().max(other.0.len()))
            .map(|i| {
                let temp = carry + self.0.get(i).unwrap_or(&0) + other.0.get(i).unwrap_or(&0);
                carry = temp / EXP;

                temp % EXP
            })
            .collect();

        if carry > 0 {
            sum.push(carry);
        }

        Self(sum)
    }
}
impl AddAssign for BigInt {
    fn add_assign(&mut self, other: Self) {
        *self = (*self).clone() + other;
    }
}

impl Ord for BigInt {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.0.len() == other.0.len() {
            self.0.iter().rev().cmp(other.0.iter().rev())
        } else {
            self.0.len().cmp(&other.0.len())
        }
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

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines();
    let mut input = || input.next().unwrap();

    let n: i32 = input().parse().unwrap();

    for (a, b) in (0..n).map(|_| (input(), input())) {
        let (a_value, b_value) = (get_paren_value(a), get_paren_value(b));
        // println!("{a_value:?} {b_value:?}");
        println!(
            "{}",
            match a_value.cmp(&b_value) {
                Ordering::Less => '<',
                Ordering::Equal => '=',
                Ordering::Greater => '>',
            }
        );
    }
}

fn get_paren_value(s: &str) -> BigInt {
    let mut paren_value = BigInt(vec![0]);
    let mut stack = String::new();
    let mut count = 0;

    for ch in s.chars() {
        stack.push(ch);

        match ch {
            '(' => count += 1,
            ')' => {
                count -= 1;

                if count == 0 {
                    paren_value += if stack == "()" {
                        BigInt(vec![1])
                    } else {
                        get_paren_value(&stack[1..stack.len() - 1]).mul(2)
                    };

                    stack.clear();
                }
            }
            _ => (),
        }
    }

    paren_value
}
