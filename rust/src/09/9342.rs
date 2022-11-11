fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    let n: i32 = buf.trim().parse().unwrap();

    'outer: for _ in 0..n {
        read_line(&mut buf);

        let mut state = Some(1);
        // https://cyberzhg.github.io/toolbox/min_dfa?regex=KEF8QnxDfER8RXxGKT9BK0YrQysoQXxCfEN8RHxFfEYpPw==
        let accept_chars = 'A'..='F';

        for c in buf.trim().chars() {
            if !accept_chars.contains(&c) {
                println!("Good");
                continue 'outer;
            }

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

#[rustfmt::skip]
fn get_state(state: Option<i32>, input: char) -> Option<i32> {
  match (state, input) {
      (Some(1), 'A') => Some(2), (Some(1), _) => Some(3),
      (Some(2), 'A') => Some(2), (Some(2), 'F') => Some(4), (Some(2), _) => None,
      (Some(3), 'A') => Some(2), (Some(3), _) => None,
      (Some(4), 'C') => Some(5), (Some(4), 'F') => Some(4), (Some(4), _) => None,
      (Some(5), 'C') => Some(5), (Some(5), _) => Some(6),
      (Some(6), _) => None,
      _ => None,
  }
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}
