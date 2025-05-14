fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let [n, m] = parse_int_vec(&buf)[..] else {
        return;
    };
    let n = n.to_string().repeat(n);

    println!("{}", &n[..m.min(n.len())]);
}

fn parse_int_vec(buf: &str) -> Vec<usize> {
    buf.split_whitespace().flat_map(str::parse).collect()
}
