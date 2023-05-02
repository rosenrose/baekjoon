fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let [s, t, d] = parse_int_vec(&buf)[..] else { return };

    println!("{}", d / (s * 2) * t);
}

fn parse_int_vec(buf: &str) -> Vec<i32> {
    buf.split_whitespace().flat_map(str::parse).collect()
}
