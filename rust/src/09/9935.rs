use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines();

    let [s, bomb] = [(); 2].map(|_| input.next().unwrap());
    let bomb_len = bomb.len();
    let mut stack = String::with_capacity(1_000_000);

    for ch in s.chars() {
        stack.push(ch);

        if &stack[stack.len().saturating_sub(bomb_len)..] == bomb {
            stack.truncate(stack.len() - bomb_len);
        }
    }

    println!("{}", if stack.is_empty() { "FRULA" } else { &stack });
}
