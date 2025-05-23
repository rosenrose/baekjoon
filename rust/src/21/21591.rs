fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let [wc, hc, ws, hs] = parse_int_vec(&buf)[..] else {
        return;
    };

    println!("{}", (wc - 2 >= ws && hc - 2 >= hs) as u8);
}

fn parse_int_vec(buf: &str) -> Vec<i32> {
    buf.split_whitespace().flat_map(str::parse).collect()
}
