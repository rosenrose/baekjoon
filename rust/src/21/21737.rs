#[derive(PartialEq)]
enum Tokens {
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
        if token == Print {
            write!(output, "{} ", stack.last().unwrap()).unwrap();
            continue;
        }
        if let Num(n) = token {
            stack.push(n);
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
            _ => unreachable!(),
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
    let mut num_idx = 0;
    let mut is_number = false;

    for (i, ch) in input.char_indices() {
        match ch {
            'S' | 'M' | 'U' | 'P' | 'C' => {
                if is_number {
                    infix.push(Num(input[num_idx..i].parse().unwrap()));
                    is_number = false;
                }

                infix.push(match ch {
                    'S' => Sub,
                    'M' => Mul,
                    'U' => Div,
                    'P' => Add,
                    'C' => Print,
                    _ => unreachable!(),
                });
            }
            '0'..='9' => {
                if !is_number {
                    num_idx = i;
                    is_number = true;
                }
            }
            _ => (),
        }
    }

    if is_number {
        infix.push(Num(input[num_idx..].parse().unwrap()));
        is_number = false;
    }

    infix
}

fn infix_to_postfix(infix: Vec<Tokens>) -> Vec<Tokens> {
    let mut stack = Vec::new();
    let mut postfix = Vec::new();

    for input in infix {
        if let Num(_) = input {
            postfix.push(input);
            continue;
        }

        while let Some(token) = stack.pop() {
            postfix.push(token);
        }

        stack.push(input)
    }

    while let Some(token) = stack.pop() {
        postfix.push(token);
    }

    postfix
}
