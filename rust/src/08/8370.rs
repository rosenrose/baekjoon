fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let [n1, k1, n2, k2] = parse_int_vec(&buf)[..] else { return };

    println!("{}", n1 * k1 + n2 * k2);
}

fn parse_int_vec(buf: &String) -> Vec<i32> {
    buf.split_whitespace().flat_map(str::parse).collect()
}
