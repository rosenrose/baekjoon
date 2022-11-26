fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    if let [n, a] = parse_int_vec(&buf)[..] {
        let (mut count, mut exp) = (0, a);

        while exp <= n {
            count += n / exp;
            exp *= a;
        }

        println!("{count}");
    }
}

fn parse_int_vec(buf: &String) -> Vec<i64> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}
