fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    if let [a, b, c, d] = parse_str_vec(&buf)[..] {
        let parse_int = |s: String| s.parse::<i64>().unwrap();

        let ab = parse_int([a, b].concat());
        let cd = parse_int([c, d].concat());

        println!("{}", ab + cd);
    }
}

fn parse_str_vec(buf: &String) -> Vec<&str> {
    buf.split_whitespace().collect()
}
