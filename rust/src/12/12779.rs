fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let [a, b] = parse_int_vec(&buf)[..] else { return };
    let range = b - a;

    let sqrt = |n: i64| {
        let n_sqrt = (n as f64).sqrt() as i64;

        if (n_sqrt - 1) * (n_sqrt - 1) <= n && n_sqrt * n_sqrt > n {
            n_sqrt - 1
        } else {
            n_sqrt
        }
    };

    let square_nums_count = sqrt(b) - sqrt(a);

    if square_nums_count == 0 {
        println!("0");
        return;
    }

    let gcd = get_gcd(square_nums_count, range);

    println!("{}/{}", square_nums_count / gcd, range / gcd);
}

fn get_gcd(mut a: i64, mut b: i64) -> i64 {
    loop {
        if b == 0 {
            return a;
        }

        (a, b) = (b, a % b);
    }
}

fn parse_int_vec(buf: &str) -> Vec<i64> {
    buf.split_whitespace().flat_map(str::parse).collect()
}
