fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let [n, w, h, l] = parse_int_vec(&buf)[..] else { return };
    let width = w / l;
    let height = h / l;

    println!("{}", n.min(width * height));
}

fn parse_int_vec(buf: &String) -> Vec<i32> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}
