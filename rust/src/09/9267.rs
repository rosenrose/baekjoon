fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    if let [a, b, s] = parse_int_vec(&buf)[..] {
        println!(
            "{}",
            if extended_euclidean(a, b, s) {
                "YES"
            } else {
                "NO"
            }
        );
    }
}

fn extended_euclidean(a: i128, b: i128, s: i128) -> bool {
    if a == s || b == s {
        return true;
    }

    if a == 0 || b == 0 || s == 0 {
        return match (a, b, s) {
            (0, 0, s) => s == 0,
            (0, b, s) => s % b == 0,
            (a, 0, s) => s % a == 0,
            (a, b, 0) => a == 0 || b == 0,
            _ => false,
        };
    }

    let (gcd, mut x, mut y) = get_ex_gcd(a, b);

    if s % gcd != 0 {
        return false;
    }

    let multiple = s / gcd;
    let (inc, dec) = (a / gcd, b / gcd);

    x *= multiple;
    y *= multiple;
    // println!("{x} {y}");
    if y <= 0 {
        let mul = y.abs() / inc;
        let mul = if inc * mul <= y.abs() { mul + 1 } else { mul };

        y += inc * mul;
        x -= dec * mul;
    }
    // println!("{x} {y}");
    while x > 0 {
        let (g, _, _) = get_ex_gcd(x, y);

        if g == 1 {
            return true;
        }

        y += inc;
        x -= dec;
    }

    false
}

fn get_ex_gcd(a: i128, b: i128) -> (i128, i128, i128) {
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
            if s1 <= 0 {
                s1 += b;
                t1 -= a;
            }

            return (r1, s1, t1);
        }
    }
}

fn parse_int_vec(buf: &String) -> Vec<i128> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}
