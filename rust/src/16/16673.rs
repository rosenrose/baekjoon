fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let [c, k, p] = parse_int_vec(&buf)[..] else {
        return;
    };

    println!("{}", (1..=c).map(|n| n * (k + p * n)).sum::<i32>());
}

fn parse_int_vec(buf: &str) -> Vec<i32> {
    buf.split_whitespace().flat_map(str::parse).collect()
}
