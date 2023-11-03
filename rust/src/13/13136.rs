fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let [r, c, n] = parse_int_vec(&buf)[..] else {
        return;
    };
    let rows = r.div_ceil(n);
    let cols = c.div_ceil(n);

    println!("{}", rows * cols);
}

fn parse_int_vec(buf: &str) -> Vec<u64> {
    buf.split_whitespace().flat_map(str::parse).collect()
}
