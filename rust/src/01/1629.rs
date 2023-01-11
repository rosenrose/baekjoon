fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let [a, b, c] = parse_int_vec(&buf)[..] else { return };

    println!("{}", pow_rem(a, b, c));
}

fn pow_rem(base: i64, exp: i64, m: i64) -> i64 {
    if exp == 1 {
        return base % m;
    }

    let mut rem = pow_rem(base, exp / 2, m);
    rem = (rem * rem) % m;

    if exp % 2 == 0 {
        rem
    } else {
        (rem * (base % m)) % m
    }
}

fn parse_int_vec(buf: &String) -> Vec<i64> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}
