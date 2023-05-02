fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let [a, b] = parse_int_vec(&buf)[..] else { return };
    let (a, b) = (a - 1, b - 1);

    let vertical = (a % 4).abs_diff(b % 4);
    let horizontal = (a / 4).abs_diff(b / 4);

    println!("{}", vertical + horizontal);
}

fn parse_int_vec(buf: &str) -> Vec<i32> {
    buf.split_whitespace().flat_map(str::parse).collect()
}
