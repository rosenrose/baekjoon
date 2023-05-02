fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let [n, m] = parse_int_vec(&buf)[..] else { return };
    let horizontal_count = n - 1;
    let vertical_count = (m - 1) * n;

    println!("{}", horizontal_count + vertical_count);
}

fn parse_int_vec(buf: &str) -> Vec<i32> {
    buf.split_whitespace().flat_map(str::parse).collect()
}
