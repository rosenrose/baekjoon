use std::fmt::Write;
use std::io;

const M: i64 = 1_000_000_007;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i64>);
    let mut input = || input.next().unwrap();
    let mut output = String::new();

    let factorial_rem = (1..=4_000_000).fold(vec![1], |mut acc, i| {
        acc.push((acc.last().unwrap() * i) % M);
        acc
    });

    for (n, k) in (0..input()).map(|_| (input(), input())) {
        writeln!(output, "{}", combination_rem(n, k, &factorial_rem)).unwrap();
    }

    print!("{output}");
}

fn combination_rem(n: i64, r: i64, factorial_rem: &Vec<i64>) -> i64 {
    if n == r || r == 0 {
        return 1;
    }

    let a = factorial_rem[n as usize];
    let b = mod_inverse_rem(
        (factorial_rem[(n - r) as usize] * factorial_rem[r as usize]) % M,
        M,
    );

    (a * b) % M
}

fn mod_inverse_rem(n: i64, modular: i64) -> i64 {
    pow_rem(n, modular - 2)
}

fn pow_rem(mut base: i64, mut exp: i64) -> i64 {
    let mut rem = 1;

    while exp > 0 {
        if exp % 2 == 1 {
            rem = (rem * base) % M;
        }

        base = (base * base) % M;
        exp /= 2;
    }

    rem
}
