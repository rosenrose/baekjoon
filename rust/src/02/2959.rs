fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let [a, b, c, d] = parse_int_vec(&buf)[..] else { return };
    let combinations = [
        (a.min(b), c.min(d)),
        (a.min(c), b.min(d)),
        (a.min(d), b.min(c)),
    ];

    let (w, h) = combinations.iter().max_by_key(|(w, h)| w * h).unwrap();

    println!("{}", w * h);
}

fn parse_int_vec(buf: &String) -> Vec<i32> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}
