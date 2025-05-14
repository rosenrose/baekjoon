fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let [m, seed, x1, x2] = parse_int_vec(&buf)[..] else {
        return;
    };

    for a in 0..m {
        for c in 0..m {
            if (a * seed + c) % m == x1 && (a * x1 + c) % m == x2 {
                println!("{a} {c}");
                return;
            }
        }
    }
}

fn parse_int_vec(buf: &str) -> Vec<i32> {
    buf.split_whitespace().flat_map(str::parse).collect()
}
