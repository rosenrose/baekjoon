fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let mut stack = Vec::new();
    let mut postfix = String::new();
    let precedence = |op: char| match op {
        '+' | '-' => 1,
        '*' | '/' => 2,
        _ => Default::default(),
    };

    for ch in buf.trim().chars() {
        match ch {
            '(' => stack.push(ch),
            ')' => loop {
                match stack.pop() {
                    Some('(') | None => break,
                    Some(token) => postfix.push(token),
                }
            },
            '+' | '-' | '*' | '/' => {
                while let Some(&token) = stack.last() {
                    if precedence(token) < precedence(ch) {
                        break;
                    }

                    postfix.push(stack.pop().unwrap());
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
