fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let [k, d, a] = parse_int_vec(&buf)[..] else {
        return;
    };

    println!(
        "{}",
        if (k + a < d) || (d == 0) {
            "hasu"
        } else {
            "gosu"
        }
    );
}

fn parse_int_vec(buf: &str) -> Vec<i32> {
    buf.trim().split('/').flat_map(str::parse).collect()
}
