fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    if let [d, h, w] = parse_float_vec(&buf)[..] {
        let multiple = d / (h * h + w * w).sqrt();

        println!("{} {}", (h * multiple).floor(), (w * multiple).floor());
    }
}

fn parse_float_vec(buf: &String) -> Vec<f64> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}
