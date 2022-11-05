fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    if let [a, b] = parse_str_vec(&buf)[..] {
        let digit_sum =
            |s: &str| -> i64 { s.chars().map(|c| c.to_digit(10).unwrap() as i64).sum() };

        println!("{}", digit_sum(a) * digit_sum(b));
    }
}

fn parse_str_vec(buf: &String) -> Vec<&str> {
    buf.split_whitespace().collect()
}
