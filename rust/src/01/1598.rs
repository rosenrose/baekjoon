fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    if let [a, b] = parse_int_vec(&buf)[..] {
        let (a, b) = (a - 1, b - 1);

        let vertical = (a % 4).abs_diff(b % 4);
        let horizontal = (a / 4).abs_diff(b / 4);

        println!("{}", vertical + horizontal);
    }
}

fn parse_int_vec(buf: &String) -> Vec<i32> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}
