fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let infix = parse_to_infix(buf.trim());
    let postfix = infix_to_postfix(infix);

    let mut stack = Vec::new();

    for token in postfix {
        if !matches!(token.as_str(), "+" | "-" | "<?" | ">?") {
            stack.push(token.parse::<i32>().unwrap());
            continue;
        }

        let (b, a) = (stack.pop().unwrap(), stack.pop().unwrap());
        let result = match token.as_str() {
            "+" => a + b,
            "-" => a - b,
            "<?" => a.min(b),
            ">?" => a.max(b),
            _ => Default::default(),
        };

        stack.push(result);
    }

    println!("{}", stack.pop().unwrap());
}

fn parse_to_infix(input: &str) -> Vec<String> {
    let mut infix = Vec::new();
    let mut number = String::new();
    let mut min_max = String::new();

    let flush = |s: &mut String, infix: &mut Vec<_>| {
        if !s.is_empty() {
            infix.push(s.clone());
            s.clear();
        }
    };

    for ch in input.chars() {
        match ch {
            '+' | '-' | '(' | ')' | '<' | '>' | '?' => {
                flush(&mut number, &mut infix);

                match ch {
                    '+' | '-' | '(' | ')' => {
                        flush(&mut min_max, &mut infix);
                        infix.push(ch.to_string());
                    }
                    '<' | '>' | '?' => min_max.push(ch),
                    _ => (),
                }
            }
            '0'..='9' => {
                flush(&mut min_max, &mut infix);
                number.push(ch);
            }
            _ => (),
        }
    }

    flush(&mut number, &mut infix);
    infix
}

fn infix_to_postfix(infix: Vec<String>) -> Vec<String> {
    let mut stack = Vec::new();
    let mut postfix = Vec::new();
    let precedence = |op: &String| match op.as_str() {
        "+" | "-" => 1,
        "<?" | ">?" => 2,
        _ => Default::default(),
    };

    for input in infix.into_iter() {
        match input.as_str() {
            "(" => stack.push(input),
            ")" => loop {
                match stack.pop() {
                    None => break,
                    Some(token) if token == "(" => break,
                    Some(token) => postfix.push(token),
                }
            },
            "+" | "-" | "<?" | ">?" => {
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

    postfix
}
