use std::cmp::Ordering;
use std::fmt;
use std::io;
use std::ops::Add;

const DIGITS: usize = 37;
const EXP: i128 = 10_i128.pow(DIGITS as u32);

#[derive(Clone, PartialEq)]
struct BigInt(Vec<i128>);

impl BigInt {
    fn parse(input: &str) -> Self {
        Self(
            input
                .as_bytes()
                .rchunks(DIGITS)
                .map(|chunk| {
                    let mut exp = 1;

                    chunk.iter().rev().fold(0, |acc, &ch| {
                        let num = (ch as i128 - '0' as i128) * exp;
                        exp *= 10;

                        acc + num
                    })
                })
                .collect(),
        )
    }

    fn is_zero(&self) -> bool {
        self.0.iter().all(|&i| i == 0)
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
        self.0.iter().rev().enumerate().for_each(|(i, num)| {
            if i == 0 {
                write!(f, "{num}").unwrap();
            } else {
                write!(f, "{num:0DIGITS$}").unwrap();
            }
        });

        write!(f, "")
    }
}

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().map(BigInt::parse);

    let mut cache = vec![BigInt(vec![1]), BigInt(vec![2])];

    while cache.last().unwrap().to_string().len() <= 100 {
        let len = cache.len();
        cache.push(cache[len - 2].clone() + cache[len - 1].clone());
    }

    while let (Some(a), Some(b)) = (input.next(), input.next()) {
        if a.is_zero() && b.is_zero() {
            return;
        }

        let start = cache.iter().position(|num| num >= &a).unwrap();
        let end = cache.iter().rposition(|num| num <= &b).unwrap();

        println!("{}", end as i32 - start as i32 + 1);
    }
}
