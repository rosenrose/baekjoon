fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let [sum, diff] = parse_int_vec(&buf)[..] else { return };
    let (a, b) = ((sum + diff) / 2, (sum - diff) / 2);

    if (a + b != sum) || (a - b != diff) || a < 0 || b < 0 || a < b {
        println!("-1");
        return;
    }

    println!("{a} {b}");
}

fn parse_int_vec(buf: &str) -> Vec<i32> {
    buf.split_whitespace().flat_map(str::parse).collect()
}
