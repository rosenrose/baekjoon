#[derive(Copy, Clone, Default)]
enum Tokens {
    #[default]
    Add,
    Sub,
    Mul,
    Div,
    Num(i32),
}

use std::io;
use Tokens::*;

const NUMS_MAX: usize = 11;
const FORMULA_MAX: usize = NUMS_MAX * 2 - 1;
const RESULT_MAX: i32 = 1_000_000_000;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let n = input.next().unwrap() as usize;
    let mut nums = [0; NUMS_MAX];

    for (i, num) in input.by_ref().take(n).enumerate() {
        nums[i] = num;
    }

    let mut operators = [Default::default(); NUMS_MAX - 1];

    for (i, op) in input
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
        .enumerate()
    {
        operators[i] = op;
    }

    let (min, max) = permutations(
        0,
        &mut [0; NUMS_MAX - 1][..n - 1],
        &mut [false; NUMS_MAX - 1],
        &operators[..n - 1],
        &nums[..n],
    );

    println!("{max}\n{min}");
}

fn permutations(
    depth: usize,
    selected: &mut [usize],
    visited_idx: &mut [bool],
    operators: &[Tokens],
    nums: &[i32],
) -> (i32, i32) {
    if depth == selected.len() {
        let mut infix = [Default::default(); FORMULA_MAX];
        infix[0] = Num(nums[0]);
        let mut infix_len = 1;

        for (i, &sel) in selected.iter().enumerate() {
            infix[infix_len] = operators[sel];
            infix_len += 1;

            infix[infix_len] = Num(nums[i + 1]);
            infix_len += 1;
        }

        let postfix = infix_to_postfix(&infix[..infix_len]);
        let result = calculate(&postfix[..infix_len]);

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
        .fold((RESULT_MAX, -RESULT_MAX), |(min, max), (i, &op)| {
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

fn infix_to_postfix(infix: &[Tokens]) -> [Tokens; FORMULA_MAX] {
    let mut stack = Vec::new();
    let mut postfix = [Default::default(); FORMULA_MAX];
    let mut postfix_len = 0;
    let precedence = |op: Tokens| match op {
        Add | Sub => 1,
        Mul | Div => 2,
        _ => unreachable!(),
    };

    for &input in infix {
        if let Num(_) = input {
            postfix[postfix_len] = input;
            postfix_len += 1;
            continue;
        }

        while let Some(&token) = stack.last() {
            if precedence(token) < precedence(input) {
                break;
            }

            postfix[postfix_len] = stack.pop().unwrap();
            postfix_len += 1;
        }

        stack.push(input);
    }

    while let Some(token) = stack.pop() {
        postfix[postfix_len] = token;
        postfix_len += 1;
    }

    postfix
}

fn calculate(postfix: &[Tokens]) -> i32 {
    let mut stack = Vec::new();

    for &token in postfix {
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
