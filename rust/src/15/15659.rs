#[derive(Copy, Clone)]
enum Tokens {
    Add,
    Sub,
    Mul,
    Div,
    Num(i32),
}

use std::io;
use Tokens::*;

const MAX: i32 = 1_000_000_000;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let n = input.next().unwrap() as usize;
    let nums: Vec<_> = input.by_ref().take(n).collect();
    let operators: Vec<_> = input
        .enumerate()
        .flat_map(|(i, count)| {
            std::iter::repeat(match i {
                0 => Add,
                1 => Sub,
                2 => Mul,
                3 => Div,
                _ => unreachable!(),
            })
            .take(count as usize)
        })
        .collect();

    let (min, max) = permutations(0, &mut vec![0; n - 1], &mut [false; 10], &operators, &nums);

    println!("{max}\n{min}");
}

fn permutations(
    depth: usize,
    selected: &mut Vec<usize>,
    visited_idx: &mut [bool],
    operators: &[Tokens],
    nums: &[i32],
) -> (i32, i32) {
    if depth == selected.len() {
        let mut infix = vec![Num(nums[0])];
        infix.extend(
            selected
                .iter()
                .enumerate()
                .flat_map(|(i, &sel)| [operators[sel], Num(nums[i + 1])]),
        );
        let postfix = infix_to_postfix(infix);
        let result = calculate(postfix);

        return (result, result);
    }

    let mut visited_op = [false; 4];
    let get_idx = |op: Tokens| match op {
        Add => 0,
        Sub => 1,
        Mul => 2,
        Div => 3,
        _ => unreachable!(),
    };

    operators
        .iter()
        .enumerate()
        .fold((MAX, -MAX), |(min, max), (i, &op)| {
            if visited_idx[i] || visited_op[get_idx(op)] {
                return (min, max);
            }

            visited_idx[i] = true;
            visited_op[get_idx(op)] = true;
            selected[depth] = i;

            let result = permutations(depth + 1, selected, visited_idx, operators, nums);
            visited_idx[i] = false;

            (result.0.min(min), result.1.max(max))
        })
}

fn infix_to_postfix(infix: Vec<Tokens>) -> Vec<Tokens> {
    let mut stack = Vec::new();
    let mut postfix = Vec::new();
    let precedence = |op: Tokens| match op {
        Add | Sub => 1,
        Mul | Div => 2,
        _ => unreachable!(),
    };

    for input in infix {
        if let Num(_) = input {
            postfix.push(input);
            continue;
        }

        while let Some(&token) = stack.last() {
            if precedence(token) < precedence(input) {
                break;
            }

            postfix.push(stack.pop().unwrap());
        }

        stack.push(input);
    }

    while let Some(token) = stack.pop() {
        postfix.push(token);
    }

    postfix
}

fn calculate(postfix: Vec<Tokens>) -> i32 {
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
            _ => unreachable!(),
        };

        stack.push(result);
    }

    stack.pop().unwrap()
}
