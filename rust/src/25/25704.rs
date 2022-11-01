fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    let n = parse_int(&buf);
    read_line(&mut buf);

    let mut p = parse_int(&buf);
    let discounts = [
        0,
        500,
        (p as f64 * 0.1) as i32,
        2000,
        (p as f64 * 0.25) as i32,
    ];

    p -= discounts[..=(n / 5).min(4) as usize]
        .into_iter()
        .max()
        .unwrap();

    println!("{}", p.max(0));
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}

fn parse_int(buf: &String) -> i32 {
    buf.trim().parse().unwrap()
}
