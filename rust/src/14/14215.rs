fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    if let [a, b, c] = parse_int_vec(&buf)[..] {
        println!(
            "{}",
            match a.max(b).max(c) {
                longest if longest == a => b + c + longest.min(b + c - 1),
                longest if longest == b => c + a + longest.min(c + a - 1),
                longest if longest == c => a + b + longest.min(a + b - 1),
                _ => 0,
            }
        );
    }
}

fn parse_int_vec(buf: &String) -> Vec<i32> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}
