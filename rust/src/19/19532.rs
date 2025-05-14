fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let [a, b, c, d, e, f] = parse_int_vec(&buf)[..] else {
        return;
    };

    let x = (c * e - f * b) / (a * e - d * b);
    let y = (c * d - f * a) / (b * d - e * a);

    println!("{x} {y}");
}

fn parse_int_vec(buf: &str) -> Vec<i32> {
    buf.split_whitespace().flat_map(str::parse).collect()
}
