use std::collections::HashSet;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    if let [a, b] = parse_int_vec(&buf)[..] {
        let diff = a.abs_diff(b) as i64;

        if diff <= 1 {
            println!("1");
            return;
        }

        let mut divisors = HashSet::new();

        for i in (1..).take_while(|i| i * i <= diff) {
            if diff % i != 0 {
                continue;
            }

            divisors.insert(i);
            divisors.insert(diff / i);
        }

        let n_lcm: HashSet<_> = divisors
            .iter()
            .map(|gcd2| {
                let mut new_a = (a / gcd2) * gcd2;

                while new_a <= a {
                    new_a += gcd2;
                }

                let n = new_a - a;

                (n, get_lcm(new_a, b + n))
            })
            .collect();

        let (min_n, _) = n_lcm.iter().min_by_key(|(_, lcm)| lcm).unwrap();

        println!("{min_n}");
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
