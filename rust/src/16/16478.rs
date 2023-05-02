fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let [ab, bc, cd] = parse_float_vec(&buf)[..] else { return };

    println!("{}", ab * cd / bc);
}

fn parse_float_vec(buf: &str) -> Vec<f64> {
    buf.split_whitespace().flat_map(str::parse).collect()
}
