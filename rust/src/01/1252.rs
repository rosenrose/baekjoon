fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    if let [a, b] = parse_str_vec(&buf)[..] {
        let a = i128::from_str_radix(a, 2).unwrap();
        let b = i128::from_str_radix(b, 2).unwrap();

        println!("{:b}", a + b);
    }
}

fn parse_str_vec(buf: &String) -> Vec<&str> {
    buf.split_whitespace().collect()
}
