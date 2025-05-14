fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let [a, t] = parse_float_vec(&buf)[..] else {
        return;
    };

    println!("{:.0}", a * a - t * t);
}

fn parse_float_vec(buf: &str) -> Vec<f64> {
    buf.split_whitespace().flat_map(str::parse).collect()
}
