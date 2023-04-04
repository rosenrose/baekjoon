use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.lines().next_back().unwrap();

    let mut stack = vec![-1];
    let mut max_len = 0;

    for (i, ch) in input.char_indices() {
        let i = i as i32;

        match ch {
            '(' => stack.push(i),
            ')' => {
                stack.pop();

                if stack.is_empty() {
                    stack.push(i);
                    continue;
                }

                max_len = max_len.max(i - stack.last().unwrap());
            }
            _ => (),
        }
    }

    println!("{max_len}");
}
