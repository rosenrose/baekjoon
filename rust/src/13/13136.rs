fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let [r, c, n] = parse_float_vec(&buf)[..] else { return };
    let rows = (r / n).ceil() as i64;
    let cols = (c / n).ceil() as i64;

    println!("{}", rows * cols);
}

fn parse_float_vec(buf: &String) -> Vec<f64> {
    buf.split_whitespace().flat_map(str::parse).collect()
}
