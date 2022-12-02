fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    let n = parse_int(buf.trim());
    read_line(&mut buf);

    let init = parse_int_vec(&buf);
    let (mut coin_a, mut coin_b) = (init[0], init[1]);

    for _ in 0..n - 1 {
        read_line(&mut buf);

        if let [a, b] = parse_int_vec(&buf)[..] {
            let lcm = get_lcm(coin_b, b);
            let gcd = get_gcd(coin_a * (lcm / coin_b), a * (lcm / b));

            (coin_a, coin_b) = (gcd, lcm);
        }
    }

    let gcd = get_gcd(coin_a, coin_b);

    println!("{} {}", coin_a / gcd, coin_b / gcd);
}

fn get_lcm(a: i64, b: i64) -> i64 {
    a / get_gcd(a, b) * b
}

fn get_gcd(mut a: i64, mut b: i64) -> i64 {
    loop {
        if b == 0 {
            return a;
        }

        (a, b) = (b, a % b);
    }
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}

fn parse_int(buf: &str) -> i64 {
    buf.parse().unwrap()
}

fn parse_int_vec(buf: &String) -> Vec<i64> {
    buf.split_whitespace().map(parse_int).collect()
}
