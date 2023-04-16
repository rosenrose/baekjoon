fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let [a, b, c, n] = parse_int_vec(&buf)[..] else { return };

    for i in 0..=n / a {
        for j in 0..=(n - a * i) / b {
            for k in 0..=(n - a * i - b * j) / c {
                if a * i + b * j + c * k == n {
                    println!("1");
                    return;
                }
            }
        }
    }

    println!("0");
}

fn parse_int_vec(buf: &String) -> Vec<i32> {
    buf.split_whitespace().flat_map(str::parse).collect()
}
