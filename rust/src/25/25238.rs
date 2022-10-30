fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    if let [a, b] = parse_float_vec(&buf)[..] {
        let net = a * ((100.0 - b) / 100.0);

        println!("{}", if net >= 100.0 { 0 } else { 1 });
    }
}

fn parse_float_vec(buf: &String) -> Vec<f64> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}