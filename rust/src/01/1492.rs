const M: usize = 1_000_000_007;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let [n, k] = parse_int_vec(&buf)[..] else { return };
    let mut power_sums = [0; 51];
    power_sums[0] = n;

    let mut combination_rem = [[0; 52]; 52];

    for i in 0..combination_rem.len() {
        combination_rem[i][0] = 1;
        combination_rem[i][i] = 1;

        for j in 1..i {
            combination_rem[i][j] = (combination_rem[i - 1][j] + combination_rem[i - 1][j - 1]) % M;
        }
    }

    for i in 1..=k {
        let mut power_sum = (pow_rem(n + 1, i + 1) - 1 + M) % M;

        for j in 0..=i - 1 {
            power_sum = (power_sum - (combination_rem[i + 1][j] * power_sums[j]) % M + M) % M;
        }

        power_sum = (power_sum * mod_inverse_rem(combination_rem[i + 1][i], M)) % M;
        power_sums[i] = power_sum;
    }

    println!("{}", power_sums[k]);
}

fn pow_rem(base: usize, exp: usize) -> usize {
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

fn mod_inverse_rem(n: usize, modular: usize) -> usize {
    pow_rem(n, modular - 2)
}

fn parse_int_vec(buf: &String) -> Vec<usize> {
    buf.split_whitespace().flat_map(str::parse).collect()
}
