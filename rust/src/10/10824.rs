fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let input = buf.split_whitespace();

    if let [a, b, c, d] = input.collect::<Vec<_>>()[..] {
        let ab = parse_int([a, b].concat());
        let cd = parse_int([c, d].concat());

        println!("{}", ab + cd);
    }
}

fn parse_int(buf: String) -> i64 {
    buf.parse().unwrap()
}
