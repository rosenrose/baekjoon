fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    let n = parse_int(buf.trim());

    for _ in 0..n {
        read_line(&mut buf);
        let players = parse_int(buf.trim());

        let (_, vip) = (0..players)
            .map(|_| {
                read_line(&mut buf);
                let mut tokens = buf.split_whitespace();

                (
                    parse_int(tokens.next().unwrap()),
                    tokens.next().unwrap().to_string(),
                )
            })
            .max_by_key(|&(guarantee, _)| guarantee)
            .unwrap();

        println!("{vip}");
    }
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}

fn parse_int(buf: &str) -> i32 {
    buf.trim().parse().unwrap()
}
