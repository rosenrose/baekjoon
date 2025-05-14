fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let [n, a] = parse_int_vec(&buf)[..] else {
        return;
    };

    println!("{} {}", n - a, mod_inverse(a, n).unwrap_or(-1));
}

fn mod_inverse(n: i64, modular: i64) -> Option<i64> {
    let (gcd, inverse, _) = get_ex_gcd(n, modular);

    (gcd == 1).then_some(inverse)
}

fn get_ex_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    let (mut r1, mut r2) = (a, b);
    let (mut s1, mut s2) = (1, 0);
    let (mut t1, mut t2) = (0, 1);
    let mut q;

    loop {
        q = r1 / r2;
        (r1, r2) = (r2, r1 % r2);
        (s1, s2) = (s2, s1 - s2 * q);
        (t1, t2) = (t2, t1 - t2 * q);

        if r2 == 0 {
            if s1 < 0 {
                s1 += b;
                t1 -= a;
            }

            return (r1, s1, t1);
        }
    }
}

fn parse_int_vec(buf: &str) -> Vec<i64> {
    buf.split_whitespace().flat_map(str::parse).collect()
}
