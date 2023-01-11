fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let [n, m, k] = parse_int_vec(&buf)[..] else { return };
    let diff = k - 3;

    let next = if diff >= 0 {
        m + diff
    } else {
        m + (n - diff.abs() % n)
    } % n;

    println!("{}", if next == 0 { n } else { next });
}

fn parse_int_vec(buf: &String) -> Vec<i32> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}
