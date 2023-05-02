fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let [mut a, b] = parse_int_vec(&buf)[..] else { return };

    print!("{}.", a / b);

    for _ in 0..1000 {
        a = a % b * 10;
        print!("{}", a / b);
    }
}

fn parse_int_vec(buf: &str) -> Vec<i32> {
    buf.split_whitespace().flat_map(str::parse).collect()
}
