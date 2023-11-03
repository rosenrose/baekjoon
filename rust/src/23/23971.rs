fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let [h, w, n, m] = parse_int_vec(&buf)[..] else {
        return;
    };
    let rows = h.div_ceil(n + 1);
    let cols = w.div_ceil(m + 1);

    println!("{}", rows * cols);
}

fn parse_int_vec(buf: &str) -> Vec<u32> {
    buf.split_whitespace().flat_map(str::parse).collect()
}
