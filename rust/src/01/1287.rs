use std::cmp::Ordering;
use std::collections::VecDeque;
use std::fmt;

#[derive(Default, PartialEq)]
struct BigInt {
    nums: VecDeque<i8>,
    sign: i8,
}

impl BigInt {
    fn new() -> Self {
        Self {
            nums: VecDeque::from([0]),
            sign: 1,
        }
    }

    fn from(nums: VecDeque<i8>, sign: i8) -> Self {
        Self { nums, sign }
    }

    fn parse(input: &str) -> Self {
        let mut result = Self {
            nums: input
                .chars()
                .rev()
                .filter_map(|c| (c != '-').then_some(c as i8 - '0' as i8))
                .collect(),
            sign: if input.starts_with('-') { -1 } else { 1 },
        };

        result.zero_justify();
        result
    }

    fn is_zero(&self) -> bool {
        self.nums.iter().all(|&i| i == 0)
    }

    fn zero_justify(&mut self) {
        while self.nums.len() > 1 && self.nums.back() == Some(&0) {
            self.nums.pop_back();
        }

        if self.is_zero() {
            self.sign = 1;
        }
    }

    fn len(&self) -> usize {
        self.nums.len()
    }

    fn abs(&self) -> Self {
        Self::from(self.nums.clone(), 1)
    }

    fn push_front(&mut self, num: i8) {
        if self.is_zero() {
            self.nums.clear();
        }

        self.nums.push_front(num);
    }

    fn add(&self, other: &Self) -> Self {
        if self.sign * other.sign == -1 {
            return if self.sign == 1 {
                self.sub(&other.abs())
            } else {
                other.sub(&self.abs())
            };
        }

        let mut carry = 0;
        let mut sum: VecDeque<_> = (0..self.len().max(other.len()))
            .map(|i| {
                let temp = carry + self.nums.get(i).unwrap_or(&0) + other.nums.get(i).unwrap_or(&0);
                carry = temp / 10;

                temp % 10
            })
            .collect();

        if carry > 0 {
            sum.push_back(carry);
        }

        Self::from(sum, self.sign)
    }

    fn sub(&self, other: &Self) -> Self {
        if self.sign * other.sign == -1 {
            return if self.sign == 1 {
                self.add(&other.abs())
            } else {
                let mut result = other.add(&self.abs());
                result.sign = -1;

                result
            };
        }

        if self < other {
            let mut result = other.sub(self);
            result.sign *= -1;

            return result;
        }

        let mut carry = 0;
        let diff: VecDeque<_> = (0..self.len().max(other.len()))
            .map(|i| {
                let temp = carry + self.nums.get(i).unwrap_or(&0) - other.nums.get(i).unwrap_or(&0);

                if temp < 0 {
                    carry = -1;
                    temp + 10
                } else {
                    carry = 0;
                    temp
                }
            })
            .collect();

        let mut result = Self::from(diff, self.sign);

        result.zero_justify();
        result
    }

    fn mul_int(&self, other: i8) -> Self {
        let mut carry = 0;
        let mut result: VecDeque<_> = self
            .nums
            .iter()
            .map(|&num| {
                let temp = carry + num * other.abs();
                carry = temp / 10;

                temp % 10
            })
            .collect();

        if carry > 0 {
            result.push_back(carry);
        }

        Self::from(result, self.sign * other.signum() as i8)
    }

    fn mul(&self, other: &Self) -> Self {
        if self.is_zero() || other.is_zero() {
            return Self::new();
        }
        if self.len() == 1 {
            return other.mul_int(self.nums[0] * self.sign);
        }
        if other.len() == 1 {
            return self.mul_int(other.nums[0] * other.sign);
        }

        let m = self.len().max(other.len()) / 2;
        let mut half = m.min(self.len());

        let y = Self::from(self.nums.range(0..half).copied().collect(), 1);
        let x = if half < self.len() {
            Self::from(self.nums.range(half..).copied().collect(), 1)
        } else {
            Self::new()
        };

        half = m.min(other.len());

        let z = Self::from(other.nums.range(0..half).copied().collect(), 1);
        let w = if half < other.len() {
            Self::from(other.nums.range(half..).copied().collect(), 1)
        } else {
            Self::new()
        };

        let (mut xw, yz) = (x.mul(&w), y.mul(&z));
        let r = x.add(&y).mul(&w.add(&z));
        let mut xz_plus_yw = r.sub(&xw.add(&yz));

        if !xw.is_zero() {
            for _ in 0..2 * m {
                xw.nums.push_front(0);
            }
        }
        if !xz_plus_yw.is_zero() {
            for _ in 0..m {
                xz_plus_yw.nums.push_front(0);
            }
        }

        Self::from((xw.add(&xz_plus_yw).add(&yz)).nums, self.sign * other.sign)
    }

    fn div(&self, other: &Self) -> Option<Self> {
        if other.is_zero() {
            return None;
        }

        let (mut dividend, mut quotient) = (Self::new(), Self::new());

        for &num in self.nums.iter().rev() {
            let mut q = 0;

            dividend.push_front(num);

            while dividend >= *other {
                dividend = dividend.sub(&other.abs());
                q += 1;
            }

            quotient.push_front(q);
        }

        if self.sign * other.sign == -1 {
            quotient.sign = -1;
        }

        if quotient.is_zero() {
            quotient.sign = 1;
        }

        Some(quotient)
    }
}

impl PartialOrd for BigInt {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.len() == other.len() {
            Some(self.nums.iter().rev().cmp(other.nums.iter().rev()))
        } else {
            Some(self.len().cmp(&other.len()))
        }
    }
}

impl fmt::Display for BigInt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.sign == -1 {
            write!(f, "-").unwrap();
        }

        self.nums.iter().rev().for_each(|num| {
            write!(f, "{num}").unwrap();
        });

        write!(f, "")
    }
}

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    if let Some(result) = calculate(buf.trim()) {
        println!("{result}");
    } else {
        println!("ROCK");
    }
}

fn calculate(input: &str) -> Option<BigInt> {
    let infix = parse_to_infix(input)?;
    // println!("{infix:?}");
    let postfix = infix_to_postfix(infix)?;
    // println!("{postfix:?}");
    let mut stack = Vec::new();

    for token in postfix {
        if !matches!(token.as_str(), "+" | "-" | "*" | "/") {
            stack.push(BigInt::parse(&token));
            continue;
        }

        let (b, a) = (stack.pop()?, stack.pop()?);
        let result = match token.as_str() {
            "+" => a.add(&b),
            "-" => a.sub(&b),
            "*" => a.mul(&b),
            "/" => a.div(&b)?,
            _ => return None,
        };

        stack.push(result);
    }

    let result = stack.pop()?;

    stack.is_empty().then_some(result)
}

fn parse_to_infix(input: &str) -> Option<Vec<String>> {
    let mut infix = Vec::new();
    let mut number = String::new();
    let mut last = '\0';

    for ch in input.chars() {
        match ch {
            '+' | '-' | '*' | '/' | '(' | ')' => {
                if matches!((last, ch), ('0'..='9', '(') | ('(', ')') | (')', '0'..='9')) {
                    return None;
                }

                if !number.is_empty() {
                    infix.push(number.clone());
                    number.clear();
                }

                infix.push(ch.to_string());
            }
            '0'..='9' => number.push(ch),
            _ => return None,
        }

        last = ch;
    }

    if !number.is_empty() {
        infix.push(number);
    }

    Some(infix)
}

fn infix_to_postfix(infix: Vec<String>) -> Option<Vec<String>> {
    let mut stack = Vec::new();
    let mut postfix = Vec::new();
    let precedence = |op: &String| match op.as_str() {
        "+" | "-" => 1,
        "*" | "/" => 2,
        _ => Default::default(),
    };

    for input in infix.into_iter() {
        match input.as_str() {
            "(" => stack.push(input),
            ")" => loop {
                match stack.pop() {
                    None => return None,
                    Some(token) if token == "(" => break,
                    Some(token) => postfix.push(token),
                }
            },
            "+" | "-" | "*" | "/" => {
                while let Some(token) = stack.last() {
                    if precedence(token) < precedence(&input) {
                        break;
                    }

                    postfix.push(stack.pop().unwrap());
                }

                stack.push(input);
            }
            _ => postfix.push(input),
        };
    }

    while let Some(token) = stack.pop() {
        postfix.push(token);
    }

    (!postfix.contains(&"(".to_owned())).then_some(postfix)
}