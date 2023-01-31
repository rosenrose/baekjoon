fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let mut stack = Vec::new();

    for c in buf.trim().chars() {
        if let (Some('('), ')') = (stack.last(), c) {
            stack.pop();
        } else {
            stack.push(c);
        }
    }

    println!("{}", stack.len());
}
