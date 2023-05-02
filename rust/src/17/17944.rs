fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let [n, t] = parse_int_vec(&buf)[..] else { return };
    let (mut num, mut delta) = (1, 1);

    for _ in 0..t - 1 {
        num += delta;

        if num == n * 2 || num == 1 {
            delta *= -1;
        }
    }

    println!("{num}");
}

fn parse_int_vec(buf: &str) -> Vec<i32> {
    buf.split_whitespace().flat_map(str::parse).collect()
}
