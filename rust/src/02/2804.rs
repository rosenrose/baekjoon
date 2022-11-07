fn main() {
  let mut buf = String::new();
  std::io::stdin().read_line(&mut buf).unwrap();

  if let [a, b] = parse_str_vec(&buf)[..] {
      let (a_index, b_index) = a
          .char_indices()
          .filter(|&(_, letter)| b.contains(letter))
          .map(|(a_idx, letter)| (a_idx, b.find(letter).unwrap()))
          .next()
          .unwrap();

      b.char_indices().for_each(|(i, letter)| {
          if i == b_index {
              println!("{a}");
          } else {
              println!(
                  "{}{letter}{}",
                  ".".repeat(a_index),
                  ".".repeat(a.len() - a_index - 1)
              );
          }
      });
  }
}

fn parse_str_vec(buf: &String) -> Vec<&str> {
  buf.split_whitespace().collect()
}
