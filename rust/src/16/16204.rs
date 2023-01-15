fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let [n, m, k] = parse_int_vec(&buf)[..] else { return };
    let o_count = m.min(k);
    let x_count = (n - m).min(n - k);

    println!("{}", o_count + x_count);
}

fn parse_int_vec(buf: &String) -> Vec<i32> {
    buf.split_whitespace().flat_map(str::parse).collect()
}
