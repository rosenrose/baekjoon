fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    let n = parse_int(&buf);
    let mut stack = Vec::new();

    for _ in 0..n {
        read_line(&mut buf);

        if buf.trim() == "0" {
            stack.pop();
        } else {
            stack.push(parse_int(&buf));
        }
    }

    println!("{}", stack.iter().sum::<i32>());
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}

fn parse_int(buf: &String) -> i32 {
    buf.trim().parse().unwrap()
}
