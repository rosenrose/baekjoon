fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    if let [a, b] = parse_int_vec(&buf)[..] {
        let sigma = |n: i64| n * (n + 1) / 2;
        let (a, b) = (a.min(b), a.max(b));

        println!("{}", sigma(b) - sigma(a - 1));
    }
}

fn parse_int_vec(buf: &String) -> Vec<i64> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}
