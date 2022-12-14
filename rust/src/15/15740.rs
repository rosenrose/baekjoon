use std::fmt;
const BILLION_SQUARE: i128 = 1_000_000_000_000_000_000;

struct BigInt {
    abs: Vec<i64>,
    sign: i8,
}

impl BigInt {
    fn from(abs: Vec<i64>, sign: i8) -> Self {
        Self { abs, sign }
    }

    fn parse(input: &str) -> Self {
        let mut sign = 1;
        let mut abs: Vec<_> = input
            .as_bytes()
            .rchunks(18)
            .map(|chunk| {
                let mut exp = 1;

                chunk.iter().rev().fold(0, |acc, &ch| {
                    if ch as char == '-' {
                        sign = -1;
                        return acc;
                    }

                    let num = (ch - '0' as u8) as i64 * exp;
                    exp *= 10;

                    acc + num
                })
            })
            .collect();

        if abs.len() > 1 && *abs.last().unwrap() == 0 {
            abs.pop();
        }

        Self { abs, sign }
    }

    fn len(&self) -> usize {
        self.abs.len()
    }
}

impl fmt::Display for BigInt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.sign == -1 {
            write!(f, "-").unwrap();
        }

        self.abs.iter().rev().enumerate().for_each(|(i, num)| {
            if i == 0 {
                write!(f, "{num}").unwrap();
            } else {
                write!(f, "{num:018}").unwrap();
            }
        });

        write!(f, "")
    }
}

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let mut nums = buf.split_whitespace();
    let a = BigInt::parse(nums.next().unwrap());
    let b = BigInt::parse(nums.next().unwrap());

    println!("{}", add(&a, &b));
}

fn add(a: &BigInt, b: &BigInt) -> BigInt {
    if a.sign * b.sign == -1 {
        return if a.sign == 1 {
            sub(a, &BigInt::from(b.abs.clone(), 1))
        } else {
            sub(b, &BigInt::from(a.abs.clone(), 1))
        };
    }

    let mut carry = 0;
    let mut sum: Vec<_> = (0..a.len().max(b.len()))
        .map(|i| {
            let temp =
                carry + *a.abs.get(i).unwrap_or(&0) as i128 + *b.abs.get(i).unwrap_or(&0) as i128;
            carry = temp / BILLION_SQUARE;

            (temp % BILLION_SQUARE) as i64
        })
        .collect();

    if carry > 0 {
        sum.push(carry as i64);
    }

    BigInt::from(sum, a.sign)
}

fn sub(a: &BigInt, b: &BigInt) -> BigInt {
    if a.sign * b.sign == -1 {
        return if a.sign == 1 {
            add(a, &BigInt::from(b.abs.clone(), 1))
        } else {
            let mut result = add(b, &BigInt::from(a.abs.clone(), 1));
            result.sign = -1;

            result
        };
    }

    if a.len() < b.len() || (a.len() == b.len() && a.abs.iter().rev().lt(b.abs.iter().rev())) {
        let mut result = sub(b, a);
        result.sign *= -1;

        return result;
    }

    let mut carry = 0;
    let mut diff: Vec<_> = (0..a.len().max(b.len()))
        .map(|i| {
            let temp = carry + *a.abs.get(i).unwrap_or(&0) - *b.abs.get(i).unwrap_or(&0);

            if temp < 0 {
                carry = -1;
                temp + BILLION_SQUARE as i64
            } else {
                carry = 0;
                temp
            }
        })
        .collect();

    while diff.len() > 1 && *diff.last().unwrap() == 0 {
        diff.pop();
    }

    let mut result = BigInt::from(diff, a.sign);

    if result.len() == 1 && result.abs[0] == 0 {
        result.sign = 1;
    }

    result
}
