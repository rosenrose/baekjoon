fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let [a, b] = parse_int_vec(&buf)[..] else {
        return;
    };
    let c = ((a * a - b) as f64).sqrt() as i32;

    if c == 0 {
        println!("{}", -a);
    } else {
        println!("{} {}", -a - c, -a + c);
    }
}

fn parse_int_vec(buf: &str) -> Vec<i32> {
    buf.split_whitespace().flat_map(str::parse).collect()
}
