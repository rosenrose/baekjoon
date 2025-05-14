fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let [n, a] = parse_int_vec(&buf)[..] else {
        return;
    };
    let (mut count, mut pow) = (0, a);

    while pow <= n {
        count += n / pow;
        pow *= a;
    }

    println!("{count}");
}

fn parse_int_vec(buf: &str) -> Vec<i64> {
    buf.split_whitespace().flat_map(str::parse).collect()
}
