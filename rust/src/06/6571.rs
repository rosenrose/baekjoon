use std::cmp::Ordering;
use std::fmt;
use std::io;
use std::ops::Add;

const DIGITS: usize = 37;
const POW: i128 = 10_i128.pow(DIGITS as u32);
const MAX: usize = 480;

#[derive(PartialEq)]
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

    fn is_zero(&self) -> bool {
        self.0.iter().all(|&i| i == 0)
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
    let mut input = buf.split_ascii_whitespace().map(BigInt::parse);

    let mut memo = [(); MAX].map(|_| BigInt(vec![0]));
    memo[0] = BigInt(vec![1]);
    memo[1] = BigInt(vec![2]);
    let mut memo_len = 2;

    while memo[memo_len - 1].to_string().len() <= 100 {
        memo[memo_len] = &memo[memo_len - 2] + &memo[memo_len - 1];
        memo_len += 1;
    }

    while let [Some(a), Some(b)] = [(); 2].map(|_| input.next()) {
        if a.is_zero() && b.is_zero() {
            return;
        }

        let start = memo[..memo_len].iter().position(|num| num >= &a).unwrap();
        let end = memo[..memo_len].iter().rposition(|num| num <= &b).unwrap();

        println!("{}", end as i32 - start as i32 + 1);
    }
}
