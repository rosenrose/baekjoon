fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let [a, b, c] = parse_int_vec(&buf)[..] else { return };
    let max = (a * b / c).max(a / b * c);

    println!("{}", max.floor());
}

fn parse_int_vec(buf: &String) -> Vec<f64> {
    buf.split_whitespace().flat_map(str::parse).collect()
}
