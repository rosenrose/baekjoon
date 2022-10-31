fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    if let [k, d, a] = parse_int_vec(&buf)[..] {
        println!(
            "{}",
            if (k + a < d) || (d == 0) {
                "hasu"
            } else {
                "gosu"
            }
        );
    }
}

fn parse_int_vec(buf: &String) -> Vec<i32> {
    buf.trim().split('/').map(|s| s.parse().unwrap()).collect()
}
