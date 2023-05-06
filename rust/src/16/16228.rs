fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let infix = parse_to_infix(buf.trim());
    let postfix = infix_to_postfix(infix);

    let mut stack = Vec::new();

    for token in postfix {
        if !matches!(token, "+" | "-" | "<?" | ">?") {
            stack.push(token.parse::<i32>().unwrap());
            continue;
        }

        let (b, a) = (stack.pop().unwrap(), stack.pop().unwrap());
        let result = match token {
            "+" => a + b,
            "-" => a - b,
            "<?" => a.min(b),
            ">?" => a.max(b),
            _ => unreachable!(),
        };

        stack.push(result);
    }

    println!("{}", stack.pop().unwrap());
}

fn parse_to_infix(input: &str) -> Vec<&str> {
    let mut infix = Vec::new();
    let regex = Regex::new(r"[0-9]+|[+-\()]|<\?|>\?", false);
    let mut cursor = 0;

    while let Some((start, end)) = regex.find(&input[cursor..]) {
        let token = &input[cursor + start..cursor + end];
        infix.push(token);

        cursor += end;
    }

    infix
}

fn infix_to_postfix(infix: Vec<&str>) -> Vec<&str> {
    let mut stack = Vec::new();
    let mut postfix = Vec::new();
    let precedence = |op: &str| match op {
        "+" | "-" => 1,
        "<?" | ">?" => 2,
        _ => 0,
    };

    for input in infix {
        match input {
            "(" => stack.push(input),
            ")" => loop {
                match stack.pop() {
                    Some("(") | None => break,
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
