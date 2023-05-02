fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let [x, y, w, h] = parse_int_vec(&buf)[..] else { return };
    let horizontal_min = x.min(w - x);
    let vertical_min = y.min(h - y);

    println!("{}", horizontal_min.min(vertical_min));
}

fn parse_int_vec(buf: &str) -> Vec<i32> {
    buf.split_whitespace().flat_map(str::parse).collect()
}
