use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

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
