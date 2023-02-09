fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let mut stack = Vec::new();
    let mut postfix = String::new();

    for ch in buf.trim().chars() {
        match ch {
            '(' => stack.push(ch),
            ')' => loop {
                match stack.pop() {
                    Some('(') | None => break,
                    Some(token) => postfix.push(token),
                }
            },
            '*' | '/' => {
                while let Some('*' | '/') = stack.last() {
                    postfix.push(stack.pop().unwrap());
                }

                stack.push(ch);
            }
            '+' | '-' => {
                loop {
                    match stack.last() {
                        Some('(') | None => break,
                        _ => postfix.push(stack.pop().unwrap()),
                    };
                }

                stack.push(ch);
            }
            _ => postfix.push(ch),
        };
    }

    while let Some(token) = stack.pop() {
        postfix.push(token);
    }

    println!("{postfix}");
}
