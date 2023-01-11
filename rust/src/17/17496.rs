fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let [n, t, c, p] = parse_int_vec(&buf)[..] else { return };
    let mut harvest_days = n / t;

    if n % t == 0 {
        harvest_days -= 1;
    }

    println!("{}", harvest_days * c * p);
}

fn parse_int_vec(buf: &String) -> Vec<i32> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}
