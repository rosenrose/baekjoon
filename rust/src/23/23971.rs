fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    if let [h, w, n, m] = parse_int_vec(&buf)[..] {
        let rows = (h as f64 / (n + 1) as f64).ceil() as i32;
        let cols = (w as f64 / (m + 1) as f64).ceil() as i32;

        println!("{}", rows * cols);
    }
}

fn parse_int_vec(buf: &String) -> Vec<i32> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}
