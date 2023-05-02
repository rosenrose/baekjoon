fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let [n, m] = parse_int_vec(&buf)[..] else { return };

    println!("{}", (n * m) / 2);
}

fn parse_int_vec(buf: &str) -> Vec<i64> {
    buf.split_whitespace().flat_map(str::parse).collect()
}
