use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut output = String::new();

    for input in buf.lines().skip(1) {
        let mut state = Some(1);

        for c in input.chars() {
            state = get_state(state, c);

            if state.is_none() {
                break;
            }
        }

        writeln!(
            output,
            "{}",
            match state {
                Some(4 | 7 | 8) => "YES",
                _ => "NO",
            }
        )
        .unwrap();
    }

    print!("{output}");
}

// https://cyberzhg.github.io/toolbox/min_dfa?regex=KDEwMCsxK3wwMSkr
#[rustfmt::skip]
fn get_state(state: Option<i32>, input: char) -> Option<i32> {
    if state.is_none() {
        return None;
    }

    match (state.unwrap(), input) {
        (1, '0') => Some(2), (1, '1') => Some(3),
        (2, '1') => Some(4),
        (3, '0') => Some(5),
        (4, '0') => Some(2), (4, '1') => Some(3),
        (5, '0') => Some(6),
        (6, '0') => Some(6), (6, '1') => Some(7),
        (7, '0') => Some(2), (7, '1') => Some(8),
        (8, '0') => Some(9), (8, '1') => Some(8),
        (9, '0') => Some(6), (9, '1') => Some(4),
        _ => None,
    }
}
