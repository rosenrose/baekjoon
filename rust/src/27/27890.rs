fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let [mut height, n] = parse_int_vec(&buf)[..] else { return };

    for _ in 0..n {
        height = if height & 1 == 0 {
            height >> 1
        } else {
            height << 1
        } ^ 6;
    }

    println!("{height}");
}

fn parse_int_vec(buf: &String) -> Vec<i32> {
    buf.split_whitespace().flat_map(str::parse).collect()
}