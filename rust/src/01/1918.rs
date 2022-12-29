fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let mut stack = Vec::new();

    for c in buf.trim().chars() {
        match c {
            '(' => stack.push(c),
            ')' => loop {
                match stack.pop() {
                    Some('(') | None => break,
                    Some(c) => print!("{c}"),
                }
            },
            '*' | '/' => {
                loop {
                    match stack.last() {
                        Some('*' | '/') => print!("{}", stack.pop().unwrap()),
                        _ => break,
                    }
                }

                stack.push(c);
            }
            '+' | '-' => {
                loop {
                    match stack.last() {
                        Some('(') | None => break,
                        _ => print!("{}", stack.pop().unwrap()),
                    };
                }

                stack.push(c);
            }
            _ => print!("{c}"),
        };
    }

    while let Some(ch) = stack.pop() {
        print!("{ch}");
    }
}
