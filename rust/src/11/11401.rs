const M: i64 = 1_000_000_007;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let [n, k] = parse_int_vec(&buf)[..] else {
        return;
    };

    println!("{}", combination_rem(n, k));
}

fn combination_rem(n: i64, r: i64) -> i64 {
    if n == r || r == 0 {
        return 1;
    }

    let mul_rem = |acc, i| (acc * (i % M)) % M;

    let a = (n - r + 1..=n).fold(1, mul_rem);
    let b = mod_inverse_rem((1..=r).fold(1, mul_rem), M);

    (a * b) % M
}

fn mod_inverse_rem(n: i64, modular: i64) -> i64 {
    pow_rem(n, modular - 2)
}

fn pow_rem(mut base: i64, mut exp: i64) -> i64 {
    let mut rem = 1;

    while exp > 0 {
        if exp & 1 == 1 {
            rem = (rem * base) % M;
        }

        base = (base * base) % M;
        exp >>= 1;
    }

    rem
}

fn parse_int_vec(buf: &str) -> Vec<i64> {
    buf.split_whitespace().flat_map(str::parse).collect()
}
