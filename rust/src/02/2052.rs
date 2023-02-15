use std::fmt;

const DIGITS: usize = 37;
const EXP: i128 = 10_i128.pow(DIGITS as u32);

struct BigInt(Vec<i128>);

impl BigInt {
    fn mul_assign(&mut self, other: i128) {
        let mut carry = 0;

        for num in self.0.iter_mut() {
            let temp = carry + *num * other;

            carry = temp / EXP;
            *num = temp % EXP;
        }

        if carry > 0 {
            self.0.push(carry);
        }
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

    let n: usize = buf.trim().parse().unwrap();
    let mut bigint = BigInt(vec![1]);

    for _ in 0..n {
        bigint.mul_assign(5);
    }

    let result = bigint.to_string();

    println!("0.{}{result}", "0".repeat(n - result.len()));
}
