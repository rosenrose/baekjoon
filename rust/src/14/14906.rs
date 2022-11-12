fn main() {
    println!("SLURPYS OUTPUT");

    let mut buf = String::new();
    read_line(&mut buf);

    let n: i32 = buf.trim().parse().unwrap();

    'outer: for _ in 0..n {
        read_line(&mut buf);
        let input = buf.trim();

        if input.len() < 5 {
            println!("NO");
            continue;
        }

        for i in 2..=input.len() - 3 {
            if is_slimp(&input[..i]) && is_slump(&input[i..]) {
                println!("YES");
                continue 'outer;
            }
        }

        println!("NO");
    }

    println!("END OF OUTPUT");
}

#[rustfmt::skip]
fn is_slump(input: &str) -> bool {
  // ((D|E)F+)+G
  // https://cyberzhg.github.io/toolbox/min_dfa?regex=KChEfEUpRispK0c=

  let mut state = Some(1);
  let accetp_chars = 'D'..='G';

  for c in input.chars() {
      if !accetp_chars.contains(&c) {
          return false;
      }

      state = match (state, c) {
          (Some(1), 'D' | 'E') => Some(2), (Some(1), _) => None,
          (Some(2), 'F') => Some(3),       (Some(2), _) => None,
          (Some(3), 'D' | 'E') => Some(2), (Some(3), 'F') => Some(3), (Some(3), 'G') => Some(4),
          (Some(4), _) => None,
          _ => None,
      };

      if state.is_none() {
          return false;
      }
  }

  match state {
      Some(4) => true,
      _ => false,
  }
}

#[rustfmt::skip]
fn is_slimp(input: &str) -> bool {
  // ((AB){m,})A(H|(slump)C)(C{m,})
  // ((AB){m,})A(H|((D|E)F+)+GC)(C{m,})
  // 양끝 AB, C의 반복수량이 서로 같아야됨
  // https://cyberzhg.github.io/toolbox/min_dfa?regex=KEFCKSpBKEh8KChEfEUpRispK0dDKUMq

  let mut state = Some(1);
  let accetp_chars = 'A'..='H';

  for c in input.chars() {
      if !accetp_chars.contains(&c) {
          return false;
      }

      state = match (state, c) {
          (Some(1), 'A') => Some(2),       (Some(1), _) => None,
          (Some(2), 'B') => Some(1),       (Some(2), 'D' | 'E') => Some(3), (Some(2), 'H') => Some(4), (Some(2), _) => None,
          (Some(3), 'F') => Some(5),       (Some(3), _) => None,
          (Some(4), 'C') => Some(4),       (Some(4), _) => None,
          (Some(5), 'D' | 'E') => Some(3), (Some(5), 'F') => Some(5),       (Some(5), 'G') => Some(6), (Some(5), _) => None,
          (Some(6), 'C') => Some(4),       (Some(6), _) => None,
          _ => None,
      };

      if state.is_none() {
          return false;
      }
  }

  if let Some(4) = state {
      let ab_count = input.matches("AB").count();
      let c_count = input.matches("C").count() - input.matches("GC").count();

      ab_count == c_count
  } else {
      false
  }
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}
