use std::collections::VecDeque;
use std::fmt;
use std::io;

const BILLION_SQUARE: i128 = 1_000_000_000_000_000_000;

struct BigInt {
    abs: VecDeque<i64>,
    sign: i8,
}

impl BigInt {
    fn new() -> Self {
        Self {
            abs: VecDeque::from([0]),
            sign: 1,
        }
    }

    fn from(abs: VecDeque<i64>, sign: i8) -> Self {
        Self { abs, sign }
    }

    fn parse(input: &str) -> Self {
        let mut sign = 1;
        let mut abs: VecDeque<_> = input
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

        if abs.len() > 1 && *abs.back().unwrap() == 0 {
            abs.pop_back();
        }

        Self { abs, sign }
    }

    fn len(&self) -> usize {
        self.abs.len()
    }

    fn is_less(&self, other: &Self) -> bool {
        if self.len() == other.len() {
            self.abs.iter().rev().lt(other.abs.iter().rev())
        } else {
            self.len() < other.len()
        }
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
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines();

    let a = BigInt::parse(input.next().unwrap());
    let b = BigInt::parse(input.next().unwrap());

    println!("{}", add(&a, &b));
    println!("{}", sub(&a, &b));
    println!("{}", karatsuba_multiply(&a, &b));
}

fn karatsuba_multiply(a: &BigInt, b: &BigInt) -> BigInt {
    // a = x * 10^m + y
    // b = w * 10^m + z
    // ab = xw * 10^2m + (xz + yw) * 10^m + yz
    let is_zero = |bigint: &BigInt| bigint.abs.iter().all(|&i| i == 0);

    if is_zero(a) || is_zero(b) {
        return BigInt::new();
    }
    if a.len() == 1 && b.len() == 1 {
        return multiply(a, b);
    }

    let m = a.len().max(b.len()) / 2;
    let mut half = m.min(a.len());

    let y = BigInt::from(a.abs.range(0..half).copied().collect(), 1);
    let x = if half < a.len() {
        BigInt::from(a.abs.range(half..).copied().collect(), 1)
    } else {
        BigInt::new()
    };

    half = m.min(b.len());
    let z = BigInt::from(b.abs.range(0..half).copied().collect(), 1);
    let w = if half < b.len() {
        BigInt::from(b.abs.range(half..).copied().collect(), 1)
    } else {
        BigInt::new()
    };

    let mut xw = karatsuba_multiply(&x, &w);
    let yz = karatsuba_multiply(&y, &z);
    let r = karatsuba_multiply(&add(&x, &y), &add(&w, &z));
    let mut xz_plus_yw = sub(&r, &add(&xw, &yz));

    if !is_zero(&xw) {
        for _ in 0..2 * m {
            xw.abs.push_front(0);
        }
    }
    if !is_zero(&xz_plus_yw) {
        for _ in 0..m {
            xz_plus_yw.abs.push_front(0);
        }
    }

    BigInt::from(add(&add(&xw, &xz_plus_yw), &yz).abs, a.sign * b.sign)
}

fn multiply(a: &BigInt, b: &BigInt) -> BigInt {
    let mut mul = VecDeque::new();
    let temp = a.abs[0] as i128 * b.abs[0] as i128;

    mul.push_back((temp % BILLION_SQUARE) as i64);

    let mut carry = temp / BILLION_SQUARE;

    while carry > 0 {
        mul.push_back((carry % BILLION_SQUARE) as i64);
        carry /= BILLION_SQUARE;
    }

    BigInt::from(mul, a.sign * b.sign)
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
    let mut sum: VecDeque<_> = (0..a.len().max(b.len()))
        .map(|i| {
            let temp =
                carry + *a.abs.get(i).unwrap_or(&0) as i128 + *b.abs.get(i).unwrap_or(&0) as i128;
            carry = temp / BILLION_SQUARE;

            (temp % BILLION_SQUARE) as i64
        })
        .collect();

    if carry > 0 {
        sum.push_back(carry as i64);
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

    if a.is_less(b) {
        let mut result = sub(b, a);
        result.sign *= -1;

        return result;
    }

    let mut carry = 0;
    let mut diff: VecDeque<_> = (0..a.len().max(b.len()))
        .map(|i| {
            let temp = carry + a.abs.get(i).unwrap_or(&0) - b.abs.get(i).unwrap_or(&0);

            if temp < 0 {
                carry = -1;
                temp + BILLION_SQUARE as i64
            } else {
                carry = 0;
                temp
            }
        })
        .collect();

    while diff.len() > 1 && *diff.back().unwrap() == 0 {
        diff.pop_back();
    }

    let mut result = BigInt::from(diff, a.sign);

    if result.len() == 1 && result.abs[0] == 0 {
        result.sign = 1;
    }

    result
}
