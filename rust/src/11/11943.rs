fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    let first = parse_int_vec(&buf);
    read_line(&mut buf);

    let second = parse_int_vec(&buf);

    if let [a, b, c, d] = [first, second].concat()[..] {
        println!("{}", (a + d).min(b + c));
    }
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}

fn parse_int_vec(buf: &String) -> Vec<i32> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}
