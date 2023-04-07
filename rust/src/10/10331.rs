use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines();

    let (infix, answer) = (
        input.next().unwrap(),
        input.next().unwrap().parse::<i32>().unwrap(),
    );
    let (multi_first, left_to_right) = infix_to_postfix(infix);

    println!(
        "{}",
        match (calculate(multi_first), calculate(left_to_right)) {
            (a, b) if a == answer && b == answer => 'U',
            (a, _) if a == answer => 'M',
            (_, b) if b == answer => 'L',
            _ => 'I',
        }
    )
}

fn calculate(postfix: String) -> i32 {
    let mut stack = Vec::new();

    for token in postfix.chars() {
        if !matches!(token, '+' | '-' | '*' | '/') {
            stack.push(token as i32 - '0' as i32);
            continue;
        }

        let (b, a) = (stack.pop().unwrap(), stack.pop().unwrap());
        let result = match token {
            '+' => a + b,
            '-' => a - b,
            '*' => a * b,
            '/' => a / b,
            _ => Default::default(),
        };

        stack.push(result);
    }

    stack.pop().unwrap()
}

fn infix_to_postfix(infix: &str) -> (String, String) {
    let mut stack = Vec::new();
    let (mut multi_first, mut left_to_right) = (String::new(), String::new());

    let precedence = |op: char| match op {
        '+' | '-' => 1,
        '*' | '/' => 2,
        _ => Default::default(),
    };

    for input in infix.chars() {
        match input {
            '+' | '-' | '*' | '/' => {
                while let Some(&token) = stack.last() {
                    if precedence(token) < precedence(input) {
                        break;
                    }

                    multi_first.push(stack.pop().unwrap());
                }

                stack.push(input);
            }
            _ => multi_first.push(input),
        };
    }

    while let Some(token) = stack.pop() {
        multi_first.push(token);
    }

    for input in infix.chars() {
        match input {
            '+' | '-' | '*' | '/' => {
                while let Some(token) = stack.pop() {
                    left_to_right.push(token);
                }

                stack.push(input);
            }
            _ => left_to_right.push(input),
        };
    }

    while let Some(token) = stack.pop() {
        left_to_right.push(token);
    }

    (multi_first, left_to_right)
}
