fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let [ur, tr, uo, to] = parse_int_vec(&buf)[..] else { return };

    println!("{}", 56 * ur + 24 * tr + 14 * uo + 6 * to);
}

fn parse_int_vec(buf: &String) -> Vec<i32> {
    buf.split_whitespace().flat_map(str::parse).collect()
}
