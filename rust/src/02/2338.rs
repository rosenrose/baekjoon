use std::collections::VecDeque;
const BILLION_SQUARE: i128 = 1_000_000_000_000_000_000;

struct BigInt {
    abs: VecDeque<i64>,
    sign: i8,
}

impl BigInt {
    fn new(abs: VecDeque<i64>, sign: i8) -> Self {
        Self { abs, sign }
    }

    fn from(input: &str) -> Self {
        let mut abs = VecDeque::new();
        let mut sign = 1;

        input.as_bytes().rchunks(18).for_each(|chunk| {
            let (mut num, mut exp) = (0, 1);

            for &c in chunk.iter().rev() {
                if c as char == '-' {
                    sign = -1;
                    continue;
                }

                num += (c - '0' as u8) as i64 * exp;
                exp *= 10;
            }

            abs.push_back(num);
        });

        if abs.len() > 1 && *abs.back().unwrap() == 0 {
            abs.pop_back();
        }

        Self { abs, sign }
    }

    fn len(&self) -> usize {
        self.abs.len()
    }
}

fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    let a = BigInt::from(buf.trim());
    read_line(&mut buf);

    let b = BigInt::from(buf.trim());

    print(&add(&a, &b));
    println!("");
    print(&sub(&a, &b));
    println!("");
    print(&karatsuba_multiply(&a, &b));
}

fn print(bigint: &BigInt) {
    if bigint.sign == -1 {
        print!("-");
    }

    bigint.abs.iter().rev().enumerate().for_each(|(i, num)| {
        if i == 0 {
            print!("{num}");
            return;
        }

        print!("{num:018}");
    })
}

fn karatsuba_multiply(a: &BigInt, b: &BigInt) -> BigInt {
    // a = x * 10^m + y
    // b = w * 10^m + z
    // ab = xw * 10^2m + (xz + yw) * 10^m + yz

    let is_zero = |bigint: &BigInt| bigint.abs.iter().all(|&i| i == 0);

    if is_zero(a) || is_zero(b) {
        return BigInt::new(VecDeque::from([0]), 1);
    }
    if a.len() == 1 && b.len() == 1 {
        return multiply(a, b);
    }

    let m = a.len().max(b.len()) / 2;
    let mut half = m.min(a.len());

    let y = BigInt::new(a.abs.range(0..half).copied().collect(), 1);
    let x = BigInt::new(
        if half < a.len() {
            a.abs.range(half..).copied().collect()
        } else {
            VecDeque::from([0])
        },
        1,
    );

    half = m.min(b.len());
    let z = BigInt::new(b.abs.range(0..half).copied().collect(), 1);
    let w = BigInt::new(
        if half < b.len() {
            b.abs.range(half..).copied().collect()
        } else {
            VecDeque::from([0])
        },
        1,
    );

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

    BigInt::new(add(&add(&xw, &xz_plus_yw), &yz).abs, a.sign * b.sign)
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

    BigInt::new(mul, a.sign * b.sign)
}

fn add(a: &BigInt, b: &BigInt) -> BigInt {
    if a.sign * b.sign == -1 {
        return if a.sign == 1 {
            sub(a, &BigInt::new(b.abs.clone(), 1))
        } else {
            sub(b, &BigInt::new(a.abs.clone(), 1))
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

    BigInt::new(sum, a.sign)
}

fn sub(a: &BigInt, b: &BigInt) -> BigInt {
    if a.sign * b.sign == -1 {
        return if a.sign == 1 {
            add(a, &BigInt::new(b.abs.clone(), 1))
        } else {
            let mut result = add(b, &BigInt::new(a.abs.clone(), 1));
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
    let mut diff: VecDeque<_> = (0..a.len().max(b.len()))
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

    while diff.len() > 1 && *diff.back().unwrap() == 0 {
        diff.pop_back();
    }

    let mut result = BigInt::new(diff, a.sign);

    if result.len() == 1 && result.abs[0] == 0 {
        result.sign = 1;
    }

    result
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}
