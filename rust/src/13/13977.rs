use std::fmt::Write;
use std::io;

const M: usize = 1_000_000_007;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut input = || input.next().unwrap();
    let mut output = String::new();

    const MAX: usize = 4_000_000;
    let mut factorial_rem = vec![0; MAX + 1];
    factorial_rem[0] = 1;

    for i in 1..=MAX {
        factorial_rem[i] = (factorial_rem[i - 1] * i) % M;
    }

    for (n, k) in (0..input()).map(|_| (input(), input())) {
        writeln!(output, "{}", combination_rem(n, k, &factorial_rem)).unwrap();
    }

    print!("{output}");
}

fn combination_rem(n: usize, r: usize, factorial_rem: &[usize]) -> usize {
    if n == r || r == 0 {
        return 1;
    }

    let numerator = factorial_rem[n];
    let denominator = (factorial_rem[n - r] * factorial_rem[r]) % M;

    (numerator * mod_inverse_rem(denominator, M)) % M
}

fn mod_inverse_rem(n: usize, modular: usize) -> usize {
    pow_rem(n, modular - 2)
}

fn pow_rem(mut base: usize, mut exp: usize) -> usize {
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
