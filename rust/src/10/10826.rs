use std::fmt;
use std::ops::Add;

const DIGITS: usize = 37;
const EXP: i128 = 10_i128.pow(DIGITS as u32);

#[derive(Clone)]
struct BigInt(Vec<i128>);

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
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let n: i128 = buf.trim().parse().unwrap();

    println!("{}", fibo(n));
}

fn fibo(n: i128) -> BigInt {
    if n <= 1 {
        return BigInt(vec![n]);
    }

    let (mut a, mut b) = (BigInt(vec![1]), BigInt(vec![1]));

    for _ in 0..n - 2 {
        (a, b) = (b.clone(), a + b);
    }

    b
}
