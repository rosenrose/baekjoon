fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let mut stack = Vec::new();

    for c in buf.trim().chars() {
        match c {
            '*' | '/' | '(' => stack.push(c),
            '+' | '-' => {
                loop {
                    match stack.last() {
                        Some('(') | None => break,
                        _ => print!("{}", stack.pop().unwrap()),
                    };
                }

                stack.push(c);
            }
            ')' => loop {
                let temp = stack.pop().unwrap();
                if temp == '(' {
                    if stack.last() == Some(&'*') || stack.last() == Some(&'/') {
                        print!("{}", stack.pop().unwrap());
                    }
                    break;
                }

                print!("{temp}");
            },
            _ => {
                print!("{c}");

                if stack.last() == Some(&'*') || stack.last() == Some(&'/') {
                    print!("{}", stack.pop().unwrap());
                }
            }
        };
    }

    while !stack.is_empty() {
        print!("{}", stack.pop().unwrap());
    }
}
