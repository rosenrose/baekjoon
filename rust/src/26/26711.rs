use std::fmt;
use std::io;
use std::ops::Add;

const DIGITS: usize = 37;
const POW: i128 = 10_i128.pow(DIGITS as u32);

struct BigInt(Vec<i128>);

impl BigInt {
    fn parse(input: &str) -> Self {
        Self(
            input
                .as_bytes()
                .rchunks(DIGITS)
                .map(|chunk| {
                    chunk
                        .iter()
                        .fold(0, |acc, ch| acc * 10 + (ch - b'0') as i128)
                })
                .collect(),
        )
    }
}

impl Add for BigInt {
    type Output = Self;

    fn add(self, other: Self) -> Self {
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

        Self(sum)
    }
}

impl fmt::Display for BigInt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, num) in self.0.iter().rev().enumerate() {
            if i == 0 {
                write!(f, "{num}")
            } else {
                write!(f, "{num:0DIGITS$}")
            }
            .unwrap();
        }

        Ok(())
    }
}

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines().map(BigInt::parse);

    let (a, b) = (input.next().unwrap(), input.next().unwrap());

    println!("{}", a + b);
}
