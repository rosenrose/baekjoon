fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    if let [ab, bc, cd] = parse_float_vec(&buf)[..] {
        println!("{}", ab * cd / bc);
    }
}

fn parse_float_vec(buf: &String) -> Vec<f32> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}