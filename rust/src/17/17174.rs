fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let [n, m] = parse_int_vec(&buf)[..] else {
        return;
    };
    let (mut count, mut pow) = (0, 1);

    while pow <= n {
        count += n / pow;
        pow *= m;
    }

    println!("{count}");
}

fn parse_int_vec(buf: &str) -> Vec<i32> {
    buf.split_whitespace().flat_map(str::parse).collect()
}
