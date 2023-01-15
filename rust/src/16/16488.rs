fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let [n, k] = parse_int_vec(&buf)[..] else { return };

    println!("{}", n * n * k);
}

fn parse_int_vec(buf: &String) -> Vec<i64> {
    buf.split_whitespace().flat_map(str::parse).collect()
}
