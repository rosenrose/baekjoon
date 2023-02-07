use std::fmt::{self, Write};
use std::io;
use std::ops::Add;

const DIGITS: usize = 37;
const EXP: i128 = 10_i128.pow(DIGITS as u32);

#[derive(Clone)]
struct BigInt(Vec<i128>);

impl BigInt {
    fn mul(&self, num: i128) -> Self {
        let mut carry = 0;
        let mut result: Vec<_> = self
            .0
            .iter()
            .map(|n| {
                let temp = carry + n * num;
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

impl fmt::Display for BigInt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, num) in self.0.iter().rev().enumerate() {
            if i == 0 {
                write!(f, "{num}").unwrap();
            } else {
                write!(f, "{num:0DIGITS$}").unwrap();
            }
        }

        write!(f, "")
    }
}

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.lines().flat_map(str::parse::<i32>);
    let mut output = String::new();

    for n in input {
        match n {
            0 | 1 => writeln!(output, "1").unwrap(),
            2 => writeln!(output, "3").unwrap(),
            _ => {
                let (mut a, mut b) = (BigInt(vec![1]), BigInt(vec![3]));

                for _ in 0..n - 2 {
                    (a, b) = (b.clone(), a.mul(2) + b);
                }

                writeln!(output, "{b}").unwrap();
            }
        }
    }

    print!("{output}");
}
