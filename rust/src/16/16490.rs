fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    if let [a, t] = parse_float_vec(&buf)[..] {
        println!("{:.0}", a * a - t * t);
    }
}

fn parse_float_vec(buf: &String) -> Vec<f32> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}