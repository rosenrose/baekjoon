use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.lines().next_back().unwrap();
    let (mut count, mut max_count) = (0_i32, 0);

    for c in input.chars() {
        match c {
            '(' => count += 1,
            ')' => count -= 1,
            _ => (),
        }

        max_count = count.abs().max(max_count);
    }

    println!("{}", if count == 0 { max_count } else { -1 });
}
