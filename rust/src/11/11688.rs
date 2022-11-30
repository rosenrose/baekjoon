fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    if let [a, b, lcm2] = parse_int_vec(&buf)[..] {
        let lcm1 = lcm(a, b);
        let mut divisors = Vec::new();

        for i in (1..).take_while(|i| i * i <= lcm2) {
            if lcm2 % i != 0 {
                continue;
            }

            divisors.push((i, lcm2 / i));
        }

        for &(c, _) in divisors.iter() {
            if lcm(lcm1, c) == lcm2 {
                println!("{c}");
                return;
            }
        }
        for &(_, c) in divisors.iter().rev() {
            if lcm(lcm1, c) == lcm2 {
                println!("{c}");
                return;
            }
        }

        println!("-1");
    }
}

fn lcm(a: i64, b: i64) -> i64 {
    a / gcd(a, b) * b
}

fn gcd(mut a: i64, mut b: i64) -> i64 {
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
