use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i64>().unwrap());
    let mut input = || input.next().unwrap();
    let mut output = String::new();

    let (a, d) = (input(), input());
    let gcd = get_gcd(a, d);

    let sequence = |n: i64| a + (n - 1) * d;
    let sum = |n: i64| n * ((n - 1) * d + 2 * a) / 2;

    for _ in 0..input() {
        let (query, left, right) = (input(), input(), input());

        let result = match query {
            1 => sum(right) - sum(left - 1),
            2 => {
                if left == right {
                    sequence(left)
                } else {
                    gcd
                }
            }
            _ => 0,
        };

        writeln!(output, "{result}").unwrap();
    }

    print!("{output}");
}

fn get_gcd(mut a: i64, mut b: i64) -> i64 {
    loop {
        if b == 0 {
            return a;
        }

        (a, b) = (b, a % b);
    }
}
