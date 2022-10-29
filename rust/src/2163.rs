fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    if let [n, m] = parse_int_vec(&buf)[..] {
        let horizontal_count = n - 1;
        let vertical_count = (m - 1) * n;

        println!("{}", horizontal_count + vertical_count);
    }
}

fn parse_int_vec(buf: &String) -> Vec<i32> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}
