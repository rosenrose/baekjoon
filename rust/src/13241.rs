fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    if let [a, b] = parse_int_vec(&buf)[..] {
        println!("{}", get_lcm(a, b));
    }
}

fn get_lcm(a: i64, b: i64) -> i64 {
    (a * b) / get_gcd(a, b)
}

fn get_gcd(mut a: i64, mut b: i64) -> i64 {
    loop {
        (a, b) = (b, a % b);

        if b == 0 {
            return a;
        }
    }
}

fn parse_int_vec(buf: &String) -> Vec<i64> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}
