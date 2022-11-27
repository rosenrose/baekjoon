use std::io::{stdin, stdout, BufWriter, Read, Write};

fn main() {
    let (stdin, stdout) = (stdin(), stdout());
    let (mut stdin, mut stdout) = (stdin.lock(), BufWriter::new(stdout.lock()));

    let mut buf = String::new();
    stdin.read_to_string(&mut buf).unwrap();

    let mut lines = buf.lines();
    let n = parse_int(lines.next().unwrap());

    for _ in 0..n {
        if let [x, y] = parse_int_vec(lines.next().unwrap())[..] {
            let gcd = get_gcd(
                lines
                    .next()
                    .unwrap()
                    .split_whitespace()
                    .skip(1)
                    .map(parse_int),
            );

            if x % gcd != 0 || y % gcd != 0 {
                writeln!(stdout, "Gave up").unwrap();
                continue;
            }

            writeln!(stdout, "Ta-da").unwrap();
        }
    }
}

fn parse_int(buf: &str) -> i32 {
    buf.parse().unwrap()
}

fn parse_int_vec(buf: &str) -> Vec<i32> {
    buf.split_whitespace().map(parse_int).collect()
}

fn get_gcd<I>(nums: I) -> i32
where
    I: Iterator<Item = i32>,
{
    nums.reduce(|mut a, mut b| loop {
        (a, b) = (b, a % b);

        if b == 0 {
            return a;
        }
    })
    .unwrap()
}
