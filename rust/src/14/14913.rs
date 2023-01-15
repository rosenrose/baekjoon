fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let [a, d, k] = parse_int_vec(&buf)[..] else { return };
    let n = (k - a) / d + 1;

    if n <= 0 || a + (n - 1) * d != k {
        println!("X");
        return;
    }

    println!("{n}");
}

fn parse_int_vec(buf: &String) -> Vec<i32> {
    buf.split_whitespace().flat_map(str::parse).collect()
}
