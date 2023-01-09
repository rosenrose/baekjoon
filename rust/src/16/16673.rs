fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    if let [c, k, p] = parse_int_vec(&buf)[..] {
        println!("{}", (1..=c).map(|n| n * (k + p * n)).sum::<i32>());
    }
}

fn parse_int_vec(buf: &String) -> Vec<i32> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}
