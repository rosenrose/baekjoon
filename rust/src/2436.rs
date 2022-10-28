fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    if let [gcd, lcm] = parse_int_vec(&buf)[..] {
        let pairs = get_pairs(gcd, lcm);

        let min_pair = pairs.iter().min_by_key(|(a, b)| a + b).unwrap();

        println!("{} {}", min_pair.0, min_pair.1);
    }
}

fn get_pairs(gcd: i64, lcm: i64) -> Vec<(i64, i64)> {
    let product = gcd * lcm;

    (gcd..=(product as f64).sqrt() as i64)
        .step_by(gcd as usize)
        .filter(|&a| {
            if product % a != 0 {
                return false;
            }

            get_gcd(a, product / a) == gcd
        })
        .map(|a| (a, product / a))
        .collect()
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
