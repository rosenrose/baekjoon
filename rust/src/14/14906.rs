use std::io::{stdin, Read};

fn main() {
    println!("SLURPYS OUTPUT");

    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    for input in buf.lines().skip(1) {
        println!("{}", if is_slurpy(input) { "YES" } else { "NO" });
    }

    println!("END OF OUTPUT");
}

// slump = ((D|E)F+)+G
// slimp = (AB)*A(H|(slump)C)C*
// (AB)*, C*의 반복수량이 서로 같아야됨
// https://cyberzhg.github.io/toolbox/min_dfa?regex=KEFCKSpBKEh8KChEfEUpRispK0dDKUMqKChEfEUpRispK0c=
#[rustfmt::skip]
fn is_slurpy(input: &str) -> bool {
    let mut state = Some(1);
    let (mut ab_count, mut c_count) = (0, 0);

    let accetp_chars = 'A'..='H';

    for c in input.chars() {
        if !accetp_chars.contains(&c) {
            return false;
        }

        state = match (state.unwrap(), c) {
            (1, 'A')       => Some(2),
            (2, 'B')       => {ab_count += 1; Some(1)},
            (2, 'D' | 'E') => Some(3), (2, 'H') => Some(4),
            (3, 'F')       => Some(5),
            (4, 'C')       => {c_count += 1; Some(4)},
            (4, 'D' | 'E') => Some(6),
            (5, 'D' | 'E') => Some(3), (5, 'F') => Some(5), (5, 'G') => Some(7),
            (6, 'F')       => Some(8),
            (7, 'C')       => Some(4),
            (8, 'D' | 'E') => Some(6), (8, 'F') => Some(8), (8, 'G') => Some(9),
            _ => None,
        };

        if state.is_none() {
            return false;
        }
    }

    if let Some(9) = state {
        ab_count == c_count
    } else {
        false
    }
}
