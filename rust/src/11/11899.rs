fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let mut stack = Vec::new();

    for c in buf.trim().chars() {
        match (stack.last(), c) {
            (Some('('), ')') => {
                stack.pop();
            }
            _ => {
                stack.push(c);
            }
        }
    }

    println!("{}", stack.len());
}
