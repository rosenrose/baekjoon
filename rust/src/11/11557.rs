fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    let n = parse_int(buf.trim());

    for _ in 0..n {
        read_line(&mut buf);
        let schools = parse_int(buf.trim());

        let (max_school, _) = (0..schools)
            .map(|_| {
                read_line(&mut buf);
                let mut tokens = buf.split_whitespace();

                (
                    tokens.next().unwrap().to_string(),
                    parse_int(tokens.next().unwrap()),
                )
            })
            .max_by_key(|&(_, alcohol)| alcohol)
            .unwrap();

        println!("{max_school}");
    }
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}

fn parse_int(buf: &str) -> i32 {
    buf.trim().parse().unwrap()
}
