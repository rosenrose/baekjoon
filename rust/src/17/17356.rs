fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let [a, b] = parse_float_vec(&buf)[..] else { return };
    let m = (b - a) / 400.0;

    println!("{:.10}", (1.0 + 10.0_f64.powf(m)).recip());
}

fn parse_float_vec(buf: &str) -> Vec<f64> {
    buf.split_whitespace().flat_map(str::parse).collect()
}
