fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    if let [c1, _, c3, _, c5, c6] = parse_int_vec(&buf)[..] {
        let (x2, x6) = (get_gcd(c1, c5), get_gcd(c3, c6));
        let (x1, x3, x5, x7) = (c1 / x2, c5 / x2, c6 / x6, c3 / x6);

        println!("{x1} {x2} {x3} {x5} {x6} {x7}");
    }
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
