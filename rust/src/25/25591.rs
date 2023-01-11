fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let [x, y] = parse_int_vec(&buf)[..] else { return };
    let (a, b) = (100 - x, 100 - y);
    let (c, d) = (100 - (a + b), a * b);
    let (q, r) = (d / 100, d % 100);

    println!("{a} {b} {c} {d} {q} {r}");
    println!("{} {r}", c + q);
}

fn parse_int_vec(buf: &String) -> Vec<i32> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}
