fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let [d, h, w] = parse_float_vec(&buf)[..] else { return };
    let multiple = d / h.hypot(w);

    println!("{} {}", (h * multiple).floor(), (w * multiple).floor());
}

fn parse_float_vec(buf: &str) -> Vec<f64> {
    buf.split_whitespace().flat_map(str::parse).collect()
}
