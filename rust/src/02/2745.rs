fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    if let [n, b] = parse_str_vec(&buf)[..] {
        let b: u32 = b.parse().unwrap();

        println!("{}", i32::from_str_radix(n, b).unwrap());
    }
}

fn parse_str_vec(buf: &String) -> Vec<&str> {
    buf.split_whitespace().collect()
}
