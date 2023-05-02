fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    match parse_int_vec(&buf)[..] {
        [_, b, c] if b >= c => println!("-1"),
        [a, b, c] => println!("{}", (a / (c - b)) + 1),
        _ => (),
    }
}

fn parse_int_vec(buf: &str) -> Vec<i32> {
    buf.split_whitespace().flat_map(str::parse).collect()
}
