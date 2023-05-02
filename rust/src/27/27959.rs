fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let [n, m] = parse_int_vec(&buf)[..] else { return };

    println!("{}", if n * 100 >= m { "Yes" } else { "No" });
}

fn parse_int_vec(buf: &str) -> Vec<i32> {
    buf.split_whitespace().flat_map(str::parse).collect()
}
