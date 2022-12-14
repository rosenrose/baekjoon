fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    if let [r, g] = parse_int_vec(&buf)[..] {
        let gcd = get_gcd(r, g);
        let mut divisors = (1..)
            .take_while(|i| i * i <= gcd)
            .fold(Vec::new(), |mut acc, i| {
                if gcd % i == 0 {
                    acc.push(i);
                    acc.push(gcd / i);
                }

                acc
            });

        divisors.dedup();

        for divisor in divisors {
            println!("{divisor} {} {}", r / divisor, g / divisor);
        }
    }
}

fn get_gcd(mut a: i32, mut b: i32) -> i32 {
    loop {
        if b == 0 {
            return a;
        }

        (a, b) = (b, a % b);
    }
}

fn parse_int_vec(buf: &String) -> Vec<i32> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}
