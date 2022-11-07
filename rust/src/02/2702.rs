use std::io::{stdin, stdout, BufRead, BufWriter, Write};

fn main() {
    let (stdin, stdout) = (stdin(), stdout());
    let (mut stdin, mut stdout) = (stdin.lock(), BufWriter::new(stdout.lock()));

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();

    let n: i32 = buf.trim().parse().unwrap();

    for _ in 0..n {
        buf.clear();
        stdin.read_line(&mut buf).unwrap();

        if let [a, b] = parse_int_vec(&buf)[..] {
            let gcd = get_gcd(a, b);
            let lcm = (a * b) / gcd;

            writeln!(stdout, "{lcm} {gcd}").unwrap();
        }
    }
}

fn get_gcd(mut a: i32, mut b: i32) -> i32 {
    loop {
        (a, b) = (b, a % b);

        if b == 0 {
            return a;
        }
    }
}

fn parse_int_vec(buf: &String) -> Vec<i32> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}
