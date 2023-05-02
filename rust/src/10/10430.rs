fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let [a, b, c] = parse_int_vec(&buf)[..] else { return };

    println!(
        "{}\n{}\n{}\n{}",
        (a + b) % c,
        ((a % c) + (b % c)) % c,
        (a * b) % c,
        ((a % c) * (b % c)) % c
    );
}

fn parse_int_vec(buf: &str) -> Vec<i32> {
    buf.split_whitespace().flat_map(str::parse).collect()
}
