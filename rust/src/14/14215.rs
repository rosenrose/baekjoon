fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let [a, b, c] = parse_int_vec(&buf)[..] else {
        return;
    };

    println!(
        "{}",
        match a.max(b).max(c) {
            longest if longest == a => b + c + longest.min(b + c - 1),
            longest if longest == b => c + a + longest.min(c + a - 1),
            longest if longest == c => a + b + longest.min(a + b - 1),
            _ => unreachable!(),
        }
    );
}

fn parse_int_vec(buf: &str) -> Vec<i32> {
    buf.split_whitespace().flat_map(str::parse).collect()
}
