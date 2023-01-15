fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let [a, b] = parse_int_vec(&buf)[..] else { return };

    println!("{}", (b * (b * (a - 2) + a)) / 2 + 1);
}

fn parse_int_vec(buf: &String) -> Vec<i64> {
    buf.split_whitespace().flat_map(str::parse).collect()
}
