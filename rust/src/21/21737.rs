#[derive(PartialEq, Default)]
enum Tokens {
    #[default]
    Print,
    Add,
    Sub,
    Mul,
    Div,
    Num(i32),
}

use std::fmt::Write;
use std::io;
use Tokens::{Add, Div, Mul, Num, Print, Sub};

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.lines().next_back().unwrap();
    let mut output = String::new();

    let infix = parse_to_infix(input);
    let postfix = infix_to_postfix(infix);
    // println!("{postfix:?}");
    let mut stack = Vec::new();

    for token in postfix {
        if let Num(n) = token {
            stack.push(n);
            continue;
        }

        if token == Print {
            write!(output, "{} ", stack.last().unwrap()).unwrap();
            continue;
        }

        let (Some(b), Some(a)) = (stack.pop(), stack.pop()) else {
            break;
        };
        let result = match token {
            Add => a + b,
            Sub => a - b,
            Mul => a * b,
            Div => a / b,
            _ => Default::default(),
        };

        stack.push(result);
    }

    println!(
        "{}",
        if output.is_empty() {
            "NO OUTPUT"
        } else {
            &output
        }
    );
}

fn parse_to_infix(input: &str) -> Vec<Tokens> {
    let mut infix = Vec::new();
    let mut number = String::new();

    for ch in input.chars() {
        match ch {
            'S' | 'M' | 'U' | 'P' | 'C' => {
                if !number.is_empty() {
                    infix.push(Num(number.parse().unwrap()));
                    number.clear();
                }

                infix.push(match ch {
                    'S' => Sub,
                    'M' => Mul,
                    'U' => Div,
                    'P' => Add,
                    'C' => Print,
                    _ => Default::default(),
                });
            }
            '0'..='9' => number.push(ch),
            _ => (),
        }
    }

    if !number.is_empty() {
        infix.push(Num(number.parse().unwrap()));
    }

    infix
}

fn infix_to_postfix(infix: Vec<Tokens>) -> Vec<Tokens> {
    let mut stack = Vec::new();
    let mut postfix = Vec::new();

    for input in infix {
        match input {
            Num(_) => postfix.push(input),
            _ => {
                while let Some(token) = stack.pop() {
                    postfix.push(token);
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
