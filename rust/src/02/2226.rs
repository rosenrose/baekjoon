use std::fmt;

const EXP: i128 = 10_000_000_000_000_000_000_000_000_000_000_000_000;

struct BigInt {
    nums: Vec<i128>,
}

impl BigInt {
    fn add(&mut self, num: i128) -> Self {
        self.nums[0] += num;
        let mut carry = self.nums[0] / EXP;

        for i in 1..self.nums.len() {
            if carry == 0 {
                break;
            }

            self.nums[i - 1] %= EXP;
            self.nums[i] += carry;
            carry = self.nums[i] / EXP;
        }

        if carry > 0 {
            self.nums.push(carry);
        }

        Self {
            nums: self.nums.clone(),
        }
    }

    fn sub(&mut self, num: i128) -> Self {
        self.nums[0] -= num;
        let mut carry = if self.nums[0] < 0 { -1 } else { 0 };

        for i in 1..self.nums.len() {
            if carry == 0 {
                break;
            }

            self.nums[i - 1] += EXP;
            self.nums[i] -= carry;
            carry = if self.nums[i] < 0 { -1 } else { 0 };
        }

        while self.nums.len() > 0 && *self.nums.last().unwrap() == 0 {
            self.nums.pop();
        }

        Self {
            nums: self.nums.clone(),
        }
    }

    fn mul(&self, num: i128) -> Self {
        let mut carry = 0;
        let mut result: Vec<_> = self
            .nums
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

        Self { nums: result }
    }
}

impl fmt::Display for BigInt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, num) in self.nums.iter().rev().enumerate() {
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

    let count = (2..=n).fold(BigInt { nums: vec![0] }, |acc, num| {
        if num % 2 == 0 {
            acc.mul(2).add(1)
        } else {
            acc.mul(2).sub(1)
        }
    });

    println!("{count}");
}
