#[derive(PartialEq, Default, Copy, Clone)]
enum Tokens {
    #[default]
    Add,
    Sub,
    Mul,
    Div,
    Num(i64),
}

use std::io;
use Tokens::{Add, Div, Mul, Num, Sub};

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();

    let precedence: Vec<_> = input.by_ref().take(4).map(parse_int).collect();
    let infix = parse_to_infix(input.next().unwrap());
    let postfix = infix_to_postfix(infix, &precedence);

    let mut stack = Vec::new();

    for token in postfix {
        if let Num(n) = token {
            stack.push(n);
            continue;
        }

        let (b, a) = (stack.pop().unwrap(), stack.pop().unwrap());
        let result = match token {
            Add => a + b,
            Sub => a - b,
            Mul => a * b,
            Div => a / b,
            _ => Default::default(),
        };

        stack.push(result);
    }

    println!("{}", stack.pop().unwrap());
}

fn parse_to_infix(input: &str) -> Vec<Tokens> {
    let mut infix = Vec::new();
    let mut number = String::new();

    for ch in input.chars().rev() {
        match ch {
            '+' | '-' | '*' | '/' => {
                if !number.is_empty() {
                    infix.push(Num(parse_int_reverse(&number)));
                    number.clear();
                }

                infix.push(match ch {
                    '+' => Add,
                    '-' => Sub,
                    '*' => Mul,
                    '/' => Div,
                    _ => Default::default(),
                });
            }
            '0'..='9' => number.push(ch),
            _ => (),
        }
    }

    if !number.is_empty() {
        infix.push(Num(parse_int_reverse(&number)));
    }

    infix
}

fn infix_to_postfix(infix: Vec<Tokens>, precedence: &Vec<i64>) -> Vec<Tokens> {
    let mut stack = Vec::new();
    let mut postfix = Vec::new();
    let precedence = |op: Tokens| match op {
        Add => precedence[0],
        Sub => precedence[1],
        Mul => precedence[2],
        Div => precedence[3],
        _ => Default::default(),
    };

    for input in infix {
        match input {
            Num(_) => postfix.push(input),
            _ => {
                while let Some(&token) = stack.last() {
                    if precedence(token) < precedence(input) {
                        break;
                    }

                    postfix.push(stack.pop().unwrap());
                }

                stack.push(input);
            }
        };
    }

    while let Some(token) = stack.pop() {
        postfix.push(token);
    }

    postfix
}

fn parse_int(buf: &str) -> i64 {
    buf.parse().unwrap()
}

fn parse_int_reverse(buf: &str) -> i64 {
    buf.chars()
        .rev()
        .fold(0, |acc, ch| acc * 10 + ch as i64 - '0' as i64)
}
