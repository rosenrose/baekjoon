fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    let n: i32 = buf.trim().parse().unwrap();

    'outer: for _ in 0..n {
        read_line(&mut buf);
        let input = buf.trim().chars();

        let mut stack = Vec::new();

        for c in input {
            match c {
                '(' => stack.push(0),
                ')' => {
                    if let None = stack.pop() {
                        println!("NO");
                        continue 'outer;
                    }
                }
                _ => (),
            };
        }

        println!("{}", if stack.is_empty() { "YES" } else { "NO" });
    }
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}
