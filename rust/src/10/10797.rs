fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    let parse_int = |s: &str| s.parse::<i32>().unwrap();

    let date = parse_int(buf.trim());
    read_line(&mut buf);

    let count = buf
        .split_whitespace()
        .map(parse_int)
        .filter(|&n| n == date)
        .count();

    println!("{count}");
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}
