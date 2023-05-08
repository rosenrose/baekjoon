use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();

    let precedence: Vec<_> = input.by_ref().take(4).map(parse_int).collect();
    let infix = parse_to_infix(input.next().unwrap());
    let postfix = infix_to_postfix(infix, precedence);

    let mut stack = Vec::new();

    for token in postfix {
        if !matches!(token, "+" | "-" | "*" | "/") {
            stack.push(parse_int(token));
            continue;
        }

        let (b, a) = (stack.pop().unwrap(), stack.pop().unwrap());
        let result = match token {
            "+" => a + b,
            "-" => a - b,
            "*" => a * b,
            "/" => a / b,
            _ => unreachable!(),
        };

        stack.push(result);
    }

    println!("{}", stack.pop().unwrap());
}

fn parse_to_infix(input: &str) -> Vec<&str> {
    let mut infix = Vec::new();
    let mut num_idx = 0;
    let mut is_number = false;

    for (i, ch) in input.char_indices().rev() {
        match ch {
            '+' | '-' | '*' | '/' => {
                if is_number {
                    infix.push(&input[i + 1..num_idx + 1]);
                    is_number = false;
                }

                infix.push(&input[i..i + 1]);
            }
            '0'..='9' => {
                if !is_number {
                    num_idx = i;
                    is_number = true;
                }
            }
            _ => unreachable!(),
        }
    }

    if is_number {
        infix.push(&input[..num_idx + 1]);
    }

    infix
}

fn infix_to_postfix(infix: Vec<&str>, precedence: Vec<i64>) -> Vec<&str> {
    let mut stack = Vec::new();
    let mut postfix = Vec::new();
    let precedence = |op: &str| match op {
        "+" => precedence[0],
        "-" => precedence[1],
        "*" => precedence[2],
        "/" => precedence[3],
        _ => unreachable!(),
    };

    for input in infix {
        match input {
            "+" | "-" | "*" | "/" => {
                while let Some(&token) = stack.last() {
                    if precedence(token) < precedence(input) {
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

    postfix
}

fn parse_int(buf: &str) -> i64 {
    buf.parse().unwrap()
}
