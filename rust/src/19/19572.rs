fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let [d1, d2, d3] = parse_float_vec(&buf)[..] else { return };
    let sum = (d1 + d2 + d3) / 2.0;
    let (a, b, c) = (sum - d3, sum - d2, sum - d1);

    if [a, b, c].iter().any(|&i| i <= 0.0) {
        println!("-1");
        return;
    }

    println!("1");
    println!("{a:.1} {b:.1} {c:.1}");
}

fn parse_float_vec(buf: &String) -> Vec<f64> {
    buf.split_whitespace().flat_map(str::parse).collect()
}
