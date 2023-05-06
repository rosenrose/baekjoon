fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let [a, b] = parse_float_vec(&buf)[..] else { return };
    let net = a * ((100.0 - b) / 100.0);

    println!("{}", u8::from(net < 100.0));
}

fn parse_float_vec(buf: &str) -> Vec<f64> {
    buf.split_whitespace().flat_map(str::parse).collect()
}
