const M: i64 = 1_000_000_007;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let [n, k] = parse_int_vec(&buf)[..] else {
        return;
    };
    const MAX: usize = 50;

    let mut combination_rem = [[0; MAX + 2]; MAX + 2];

    for i in 0..=k + 1 {
        combination_rem[i][0] = 1;
        combination_rem[i][i] = 1;

        for j in 1..i {
            combination_rem[i][j] = (combination_rem[i - 1][j] + combination_rem[i - 1][j - 1]) % M;
        }
    }

    let mut power_sums = [0; MAX + 1];
    power_sums[0] = n as i64;

    for i in 1..=k {
        let mut power_sum = (pow_rem(n as i64 + 1, i as i64 + 1) - 1).rem_euclid(M);

        for j in 0..=i - 1 {
            power_sum = (power_sum - (combination_rem[i + 1][j] * power_sums[j]) % M).rem_euclid(M);
        }

        power_sum = (power_sum * mod_inverse_rem(combination_rem[i + 1][i], M)) % M;
        power_sums[i] = power_sum;
    }

    println!("{}", power_sums[k]);
}

fn pow_rem(base: i64, exp: i64) -> i64 {
    if exp == 1 {
        return base % M;
    }

    let mut rem = pow_rem(base, exp >> 1);
    rem = (rem * rem) % M;

    if exp & 1 == 0 {
        rem
    } else {
        (rem * (base % M)) % M
    }
}

fn mod_inverse_rem(n: i64, modular: i64) -> i64 {
    pow_rem(n, modular - 2)
}

fn parse_int_vec(buf: &str) -> Vec<usize> {
    buf.split_whitespace().flat_map(str::parse).collect()
}
