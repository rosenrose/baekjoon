use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let accept_chars = 'A'..='F';

    for input in buf.lines().skip(1) {
        if input.chars().any(|c| !accept_chars.contains(&c)) {
            println!("Good");
            continue;
        }

        let mut state = Some(1);

        for c in input.chars() {
            state = get_state(state, c);

            if state.is_none() {
                break;
            }
        }

        match state {
            Some(5 | 6) => println!("Infected!"),
            _ => println!("Good"),
        }
    }
}

// https://cyberzhg.github.io/toolbox/min_dfa?regex=KEF8QnxDfER8RXxGKT9BK0YrQysoQXxCfEN8RHxFfEYpPw==
#[rustfmt::skip]
fn get_state(state: Option<i32>, input: char) -> Option<i32> {
    if state.is_none() {
        return None;
    }

    match (state.unwrap(), input) {
        (1, 'A') => Some(2), (1, _) => Some(3),
        (2, 'A') => Some(2), (2, 'F') => Some(4),
        (3, 'A') => Some(2),
        (4, 'C') => Some(5), (4, 'F') => Some(4),
        (5, 'C') => Some(5), (5, _) => Some(6),
        _ => None,
    }
}
