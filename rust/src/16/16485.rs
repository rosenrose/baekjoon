fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let [c, b] = parse_float_vec(&buf)[..] else { return };

    println!("{}", c / b);
}

fn parse_float_vec(buf: &String) -> Vec<f64> {
    buf.split_whitespace().flat_map(str::parse).collect()
}
