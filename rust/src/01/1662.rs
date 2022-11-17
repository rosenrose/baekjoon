fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let s = buf.trim();

    println!("{}", unzipped_len(s));
}

fn unzipped_len(input: &str) -> usize {
    let mut len = 0;
    let mut paren_stack = Vec::new();
    let mut paren_str = String::new();
    let mut repeat_times = 0;

    for c in input.chars() {
        match c {
            '(' => {
                if paren_stack.is_empty() {
                    len -= 1;
                } else {
                    paren_str.push(c);
                }

                paren_stack.push(c);
            }
            ')' => {
                paren_stack.pop();

                if paren_stack.is_empty() {
                    len += repeat_times * unzipped_len(&paren_str[..]);
                    paren_str.clear();
                } else {
                    paren_str.push(c);
                }
            }
            _ => {
                if paren_stack.is_empty() {
                    len += 1;
                    repeat_times = c.to_digit(10).unwrap() as usize;
                } else {
                    paren_str.push(c);
                }
            }
        }
    }

    len
}
