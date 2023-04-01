use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let name = buf.lines().next_back().unwrap();

    let mut stack = Vec::new();
    let mut indices = Vec::new();

    for (i, ch) in name.char_indices() {
        match ch {
            '(' => stack.push(i),
            ')' => {
                let Some(mut open_idx) = stack.pop() else {
                    continue;
                };
                let close_idx = i;

                while let Some(&(o, c)) = indices.last() {
                    if open_idx < o && c < close_idx {
                        indices.pop();
                    } else {
                        break;
                    }
                }

                if let Some(&(o, c)) = indices.last() {
                    if c + 1 == open_idx {
                        open_idx = o;
                        indices.pop();
                    }
                }

                indices.push((open_idx, close_idx));
            }
            _ => (),
        }
    }
    // println!("{indices:?}");
    let max_len = indices
        .iter()
        .map(|(open, close)| close - open + 1)
        .max()
        .unwrap_or(0);

    println!("{max_len}");
}
