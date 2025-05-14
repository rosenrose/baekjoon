fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let [r1, s] = parse_int_vec(&buf)[..] else {
        return;
    };

    println!("{}", s * 2 - r1);
}

fn parse_int_vec(buf: &str) -> Vec<i32> {
    buf.split_whitespace().flat_map(str::parse).collect()
}
