use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();

    for input in buf.lines().take_while(|&input| input != ".") {
        let postfix = infix_to_postfix(input);
        let mut count = 0;
        // println!("{postfix}");
        for p in 0..=2 {
            for q in 0..=2 {
                for r in 0..=2 {
                    if calculate((p, q, r), &postfix) == 2 {
                        count += 1;
                    }
                }
            }
        }

        println!("{count}");
    }
}

fn infix_to_postfix(input: &str) -> String {
    let mut stack = Vec::new();
    let mut postfix = String::new();

    for ch in input.chars() {
        match ch {
            '(' | '-' | '*' | '+' => stack.push(ch),
            ')' => {
                loop {
                    match stack.pop() {
                        Some('(') | None => break,
                        Some(token) => postfix.push(token),
                    }
                }

                while let Some('-') = stack.last() {
                    postfix.push(stack.pop().unwrap());
                }
            }
            _ => {
                postfix.push(ch);

                while let Some('-') = stack.last() {
                    postfix.push(stack.pop().unwrap());
                }
            }
        };
    }

    while let Some(token) = stack.pop() {
        postfix.push(token);
    }

    postfix
}

fn calculate((p, q, r): (u8, u8, u8), postfix: &str) -> u8 {
    let mut stack = Vec::new();

    for token in postfix.chars() {
        match token {
            '0' | '1' | '2' => stack.push(token as u8 - '0' as u8),
            'P' => stack.push(p),
            'Q' => stack.push(q),
            'R' => stack.push(r),
            '-' => {
                let a = stack.pop().unwrap();
                stack.push(not(a));
            }
            '*' => {
                let (b, a) = (stack.pop().unwrap(), stack.pop().unwrap());
                stack.push(and(a, b));
            }
            '+' => {
                let (b, a) = (stack.pop().unwrap(), stack.pop().unwrap());
                stack.push(or(a, b));
            }
            _ => (),
        }
    }

    stack.pop().unwrap()
}

fn not(x: u8) -> u8 {
    match x {
        0 => 2,
        1 => 1,
        2 => 0,
        _ => unreachable!(),
    }
}
fn and(x: u8, y: u8) -> u8 {
    match (x, y) {
        (0, _) | (_, 0) => 0,
        (1, _) | (_, 1) => 1,
        (2, 2) => 2,
        _ => unreachable!(),
    }
}
fn or(x: u8, y: u8) -> u8 {
    match (x, y) {
        (2, _) | (_, 2) => 2,
        (1, _) | (_, 1) => 1,
        (0, 0) => 0,
        _ => unreachable!(),
    }
}
