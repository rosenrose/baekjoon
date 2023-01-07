fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    if let [k, n] = parse_int_vec(&buf)[..] {
        if n == 1 {
            println!("-1");
            return;
        }

        for x in k * n / (n - 1).. {
            if (x - k) * n >= x {
                println!("{x}");
                return;
            }
        }
    }
}

fn parse_int_vec(buf: &String) -> Vec<i64> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}
