fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let [n, m] = parse_int_vec(&buf)[..] else {
        return;
    };

    let combination = (1..=m).fold((1, 1), |mut acc, num| {
        acc = (acc.0 * (n - m + num), acc.1 * num);
        let gcd = get_gcd(acc.0, acc.1);

        (acc.0 / gcd, acc.1 / gcd)
    });

    println!("{}", combination.0 / combination.1);
}

fn get_gcd(mut a: i128, mut b: i128) -> i128 {
    loop {
        if b == 0 {
            return a;
        }

        (a, b) = (b, a % b);
    }
}

fn parse_int_vec(buf: &str) -> Vec<i128> {
    buf.split_whitespace().flat_map(str::parse).collect()
}
