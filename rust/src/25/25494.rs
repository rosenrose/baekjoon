fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    let parse_int = |s: &str| s.parse::<i32>().unwrap();
    let n = parse_int(buf.trim());

    for _ in 0..n {
        read_line(&mut buf);

        println!("{}", buf.split_whitespace().map(parse_int).min().unwrap());
    }
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}
