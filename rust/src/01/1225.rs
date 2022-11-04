fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    if let [a, b] = parse_str_vec(&buf)[..] {
        let a_digit_sum: i64 = a.chars().map(|c| c.to_digit(10).unwrap() as i64).sum();

        let result: i64 = b
            .chars()
            .map(|c| c.to_digit(10).unwrap() as i64 * a_digit_sum)
            .sum();

        println!("{result}");
    }
}

fn parse_str_vec(buf: &String) -> Vec<&str> {
    buf.split_whitespace().collect()
}
