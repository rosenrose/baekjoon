use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.lines().next_back().unwrap();

    let (mut depth, mut max_depth) = (0_i32, 0);

    for c in input.chars() {
        match c {
            '(' => depth += 1,
            ')' => depth -= 1,
            _ => (),
        }

        max_depth = depth.abs().max(max_depth);
    }

    println!("{}", if depth == 0 { max_depth } else { -1 });
}
