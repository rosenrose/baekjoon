use std::fmt::{self, Write};
use std::io;
use std::ops::Add;

const DIGITS: usize = 37;
const POW: i128 = 10_i128.pow(DIGITS as u32);

struct BigInt(Vec<i128>);

impl BigInt {
    fn mul(&self, other: i128) -> Self {
        let mut carry = 0;
        let mut result: Vec<_> = self
            .0
            .iter()
            .map(|num| {
                let temp = carry + num * other;
                carry = temp / POW;

                temp % POW
            })
            .collect();

        if carry > 0 {
            result.push(carry);
        }

        Self(result)
    }
}

impl Add for &BigInt {
    type Output = BigInt;

    fn add(self, other: Self) -> Self::Output {
        let mut carry = 0;
        let mut sum: Vec<_> = (0..self.0.len().max(other.0.len()))
            .map(|i| {
                let temp = carry + self.0.get(i).unwrap_or(&0) + other.0.get(i).unwrap_or(&0);
                carry = temp / POW;

                temp % POW
            })
            .collect();

        if carry > 0 {
            sum.push(carry);
        }

        BigInt(sum)
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

        Ok(())
    }
}

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.lines().flat_map(str::parse::<i32>);
    let mut output = String::new();

    for n in input {
        (match n {
            0 | 1 => writeln!(output, "1"),
            2 => writeln!(output, "3"),
            _ => {
                let (mut a, mut b) = (BigInt(vec![1]), BigInt(vec![3]));

                for _ in 0..n - 2 {
                    let next = &a.mul(2) + &b;
                    (a, b) = (b, next);
                }

                writeln!(output, "{b}")
            }
        })
        .unwrap();
    }

    print!("{output}");
}
