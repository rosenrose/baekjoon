use std::io::{stdin, stdout, BufWriter, Read, Write};

fn main() {
    let (stdin, stdout) = (stdin(), stdout());
    let (mut stdin, mut stdout) = (stdin.lock(), BufWriter::new(stdout.lock()));

    let mut buf = String::new();
    stdin.read_to_string(&mut buf).unwrap();

    buf.lines().skip(1).for_each(|line| {
        let n = parse_int(line);
        let lcm1 = get_lcm(n, n - 1);
        let lcm2 = get_lcm(n - 2, (n - 3).max(1));

        let lcm_list = [
            get_lcm(lcm1, n - 2),
            get_lcm(lcm1, (n - 3).max(1)),
            get_lcm(n, lcm2),
            get_lcm(n - 1, lcm2),
        ];

        writeln!(stdout, "{}", lcm_list.iter().max().unwrap()).unwrap();
    });
}

fn get_lcm(a: i64, b: i64) -> i64 {
    a / get_gcd(a, b) * b
}

fn get_gcd(mut a: i64, mut b: i64) -> i64 {
    loop {
        if b == 0 {
            return a;
        }

        (a, b) = (b, a % b);
    }
}

fn parse_int(buf: &str) -> i64 {
    buf.parse().unwrap()
}
