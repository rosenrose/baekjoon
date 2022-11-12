fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let mut state = Some(1);
    // https://cyberzhg.github.io/toolbox/min_dfa?regex=KDEwMCsxK3wwMSkr

    for c in buf.trim().chars() {
        state = get_state(state, c);

        if state.is_none() {
            break;
        }
    }

    match state {
        Some(4 | 7 | 8) => println!("SUBMARINE"),
        _ => println!("NOISE"),
    }
}

#[rustfmt::skip]
fn get_state(state: Option<i32>, input: char) -> Option<i32> {
  match (state, input) {
      (Some(1), '0') => Some(2), (Some(1), '1') => Some(3),
      (Some(2), '1') => Some(4),
      (Some(3), '0') => Some(5),
      (Some(4), '0') => Some(2), (Some(4), '1') => Some(3),
      (Some(5), '0') => Some(6),
      (Some(6), '0') => Some(6), (Some(6), '1') => Some(7),
      (Some(7), '0') => Some(2), (Some(7), '1') => Some(8),
      (Some(8), '0') => Some(9), (Some(8), '1') => Some(8),
      (Some(9), '0') => Some(6), (Some(9), '1') => Some(4),
      _ => None,
  }
}
