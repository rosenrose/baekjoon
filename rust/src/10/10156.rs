fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let [k, n, m] = parse_int_vec(&buf)[..] else { return };

    println!("{}", (k * n - m).max(0));
}

fn parse_int_vec(buf: &String) -> Vec<i32> {
    buf.split_whitespace().flat_map(str::parse).collect()
}
