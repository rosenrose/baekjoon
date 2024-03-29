use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i64>);
    let mut output = String::new();

    while let (Some(b @ 1..), Some(n @ 1..)) = (input.next(), input.next()) {
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
