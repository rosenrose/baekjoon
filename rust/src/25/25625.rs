fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let [x, y] = parse_int_vec(&buf)[..] else { return };

    println!("{}", if x < y { y - x } else { x + y });
}

fn parse_int_vec(buf: &str) -> Vec<i32> {
    buf.split_whitespace().flat_map(str::parse).collect()
}
