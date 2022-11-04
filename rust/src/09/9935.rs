fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    let input = buf.trim().to_string();
    read_line(&mut buf);

    let bomb = buf.trim();
    let bomb_len = bomb.len();

    let mut stack = String::new();

    for c in input.chars() {
        stack.push(c);

        if stack.len() < bomb_len {
            continue;
        }

        if stack[stack.len() - bomb_len..stack.len()] == bomb[..] {
            stack.truncate(stack.len() - bomb_len);
        }
    }

    println!(
        "{}",
        if stack.is_empty() {
            "FRULA".to_string()
        } else {
            stack
        }
    );
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}
