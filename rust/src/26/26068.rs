fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    let n = parse_int(buf.trim());
    let count = (0..n)
        .filter(|_| {
            read_line(&mut buf);
            parse_int(buf.trim().split('-').next_back().unwrap()) <= 90
        })
        .count();

    println!("{count}");
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}

fn parse_int(buf: &str) -> i32 {
    buf.parse().unwrap()
}
