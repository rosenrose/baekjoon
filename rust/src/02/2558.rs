fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    let a = parse_int(&buf);
    read_line(&mut buf);

    let b = parse_int(&buf);

    println!("{}", a + b);
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}

fn parse_int(buf: &String) -> i32 {
    buf.trim().parse().unwrap()
}
