use std::fmt;

const EXP: i128 = 10_000_000_000_000_000_000_000_000_000_000_000_000;

struct BigInt(Vec<i128>);

impl BigInt {
    fn add(&self, num: i128) -> Self {
        let mut carry = 0;
        let mut result = self.0.clone();
        result[0] += num;

        for num in result.iter_mut() {
            carry = (carry + *num) / EXP;
            *num %= EXP;
        }

        if carry > 0 {
            result.push(carry);
        }

        Self(result)
    }

    fn sub(&self, num: i128) -> Self {
        let mut carry = 0;
        let mut result = self.0.clone();
        result[0] -= num;

        for num in result.iter_mut() {
            *num += carry;

            if *num < 0 {
                carry = -1;
                *num += EXP;
            } else {
                carry = 0;
            }
        }

        while result.len() > 0 && *result.last().unwrap() == 0 {
            result.pop();
        }

        Self(result.clone())
    }

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

impl fmt::Display for BigInt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, num) in self.0.iter().rev().enumerate() {
            if i == 0 {
                write!(f, "{num}").unwrap();
            } else {
                write!(f, "{num:037}").unwrap();
            }
        }

        write!(f, "")
    }
}

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let n: i128 = buf.trim().parse().unwrap();
    // let bin = (0..n - 1).fold(1.to_string(), |acc, _| {
    //     acc.chars()
    //         .map(|c| if c == '0' { "10" } else { "01" })
    //         .collect()
    // });
    // println!("{bin}");
    let count = (2..=n).fold(BigInt(vec![0]), |acc, num| {
        if num % 2 == 0 {
            acc.mul(2).add(1)
        } else {
            acc.mul(2).sub(1)
        }
    });

    println!("{count}");
}
