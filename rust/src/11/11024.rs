fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    let n = parse_int(buf.trim());

    for _ in 0..n {
        read_line(&mut buf);
        let sum: i32 = buf.split_whitespace().map(parse_int).sum();

        println!("{sum}");
    }
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}

fn parse_int(buf: &str) -> i32 {
    buf.trim().parse().unwrap()
}
