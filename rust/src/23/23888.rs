use std::io::{stdin, stdout, BufWriter, Read, Write};

fn main() {
    let (stdin, stdout) = (stdin(), stdout());
    let (mut stdin, mut stdout) = (stdin.lock(), BufWriter::new(stdout.lock()));

    let mut buf = String::new();
    stdin.read_to_string(&mut buf).unwrap();

    let mut lines = buf.lines();
    let sequence = parse_int_vec(lines.next().unwrap());
    let (a, d) = (sequence[0], sequence[1]);
    let gcd = get_gcd(a, d);

    let sequence = |n: i64| a + (n - 1) * d;
    let sum = |n: i64| n * ((n - 1) * d + 2 * a) / 2;

    lines.skip(1).for_each(|line| {
        if let [q, left, right] = parse_int_vec(line)[..] {
            let result = match q {
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

            writeln!(stdout, "{result}").unwrap();
        }
    });
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
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}
