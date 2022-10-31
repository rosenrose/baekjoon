fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    let d1 = parse_float(&buf);
    read_line(&mut buf);

    let d2 = parse_float(&buf);

    println!("{:.10}", (d1 * 2.0) + (2.0 * 3.141592 * d2));
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}

fn parse_float(buf: &String) -> f64 {
    buf.trim().parse().unwrap()
}
