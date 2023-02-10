fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    println!("{}", get_paren_value(buf.trim()).unwrap_or(0));
}

fn get_paren_value(s: &str) -> Option<i64> {
    let mut paren_value = 0;
    let mut stack = String::new();
    let mut validation = Vec::new();

    for ch in s.chars() {
        stack.push(ch);

        match ch {
            '(' | '[' => validation.push(ch),
            ')' | ']' => {
                if matches!((validation.pop()?, ch), ('(', ']') | ('[', ')')) {
                    return None;
                }

                if validation.is_empty() {
                    paren_value += match stack.as_str() {
                        "()" => 2,
                        "[]" => 3,
                        _ => {
                            get_paren_value(&stack[1..stack.len() - 1])?
                                * if stack.starts_with('(') { 2 } else { 3 }
                        }
                    };

                    stack.clear();
                }
            }
            _ => (),
        }
    }

    if validation.is_empty() {
        Some(paren_value)
    } else {
        None
    }
}
