fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let [wc, hc, ws, hs] = parse_int_vec(&buf)[..] else { return };

    println!("{}", if wc - 2 >= ws && hc - 2 >= hs { 1 } else { 0 });
}

fn parse_int_vec(buf: &String) -> Vec<i32> {
    buf.split_whitespace().flat_map(str::parse).collect()
}
