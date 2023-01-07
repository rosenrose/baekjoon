use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();

    let bomb = input.next_back().unwrap();
    let bomb_len = bomb.len();
    let input = input.next().unwrap();

    let mut stack = String::new();

    for c in input.chars() {
        stack.push(c);

        if stack[stack.len().saturating_sub(bomb_len)..] == bomb[..] {
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
