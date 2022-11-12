fn main() {
    println!("SLURPYS OUTPUT");

    let mut buf = String::new();
    read_line(&mut buf);

    let n: i32 = buf.trim().parse().unwrap();

    for _ in 0..n {
        read_line(&mut buf);

        println!("{}", if is_slurpy(buf.trim()) { "YES" } else { "NO" });
    }

    println!("END OF OUTPUT");
}

#[rustfmt::skip]
fn is_slurpy(input: &str) -> bool {
  // slump = ((D|E)F+)+G
  // slimp = (AB)*A(H|(slump)C)C*
  // (AB)*, C*의 반복수량이 서로 같아야됨
  // https://cyberzhg.github.io/toolbox/min_dfa?regex=KEFCKSpBKEh8KChEfEUpRispK0dDKUMqKChEfEUpRispK0c=

  let mut state = Some(1);
  let accetp_chars = 'A'..='H';

  for c in input.chars() {
      if !accetp_chars.contains(&c) {
          return false;
      }

      state = match (state, c) {
          (Some(1), 'A')       => Some(2),
          (Some(2), 'B')       => Some(1), (Some(2), 'D' | 'E') => Some(3), (Some(2), 'H') => Some(4),
          (Some(3), 'F')       => Some(5),
          (Some(4), 'C')       => Some(4), (Some(4), 'D' | 'E') => Some(6),
          (Some(5), 'D' | 'E') => Some(3), (Some(5), 'F')       => Some(5), (Some(5), 'G') => Some(7),
          (Some(6), 'F')       => Some(8),
          (Some(7), 'C')       => Some(4),
          (Some(8), 'D' | 'E') => Some(6), (Some(8), 'F')       => Some(8), (Some(8), 'G') => Some(9),
          _ => None,
      };

      if state.is_none() {
          return false;
      }
  }

  if let Some(9) = state {
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
