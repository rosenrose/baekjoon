use std::collections::BTreeSet;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    if let [a, b, lcm2] = parse_int_vec(&buf)[..] {
        let lcm1 = get_lcm(a, b);

        let divisors = (1..)
            .take_while(|i| i * i <= lcm2)
            .fold(BTreeSet::new(), |mut acc, i| {
                if lcm2 % i == 0 {
                    acc.insert(i);
                    acc.insert(lcm2 / i);
                }

                acc
            });

        for c in divisors {
            if get_lcm(lcm1, c) == lcm2 {
                println!("{c}");
                return;
            }
        }

        println!("-1");
    }
}

fn get_lcm(a: i64, b: i64) -> i64 {
    a / get_gcd(a, b) * b
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
