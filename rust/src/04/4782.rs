use std::fmt::Write;
use std::io::{stdin, Read};

fn main() {
    let stdin = stdin();
    let mut stdin = stdin.lock();

    let mut buf = String::new();
    stdin.read_to_string(&mut buf).unwrap();

    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i64>().unwrap());
    let mut output = String::new();

    loop {
        let (b, n) = (input.next().unwrap(), input.next().unwrap());

        if (b, n) == (0, 0) {
            break;
        }

        let step = n / get_gcd(b, n);

        for m in (step..=2 * n)
            .rev()
            .step_by(step as usize)
            .filter(|&m| m != n)
        {
            let dividend = b * m * (2 * n - m);
            let divisor = n * n;

            if dividend % divisor != 0 {
                continue;
            }

            let a = dividend / divisor;

            write!(output, "{a}/{m} ").unwrap();
        }

        writeln!(output, "").unwrap();
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
