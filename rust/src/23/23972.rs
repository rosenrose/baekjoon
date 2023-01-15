fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let [k, n] = parse_int_vec(&buf)[..] else { return };

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

fn parse_int_vec(buf: &String) -> Vec<i64> {
    buf.split_whitespace().flat_map(str::parse).collect()
}
