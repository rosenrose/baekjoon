use std::collections::HashSet;
use std::io::{stdin, stdout, BufWriter, Read, Write};

fn main() {
    let (stdin, stdout) = (stdin(), stdout());
    let (mut stdin, mut stdout) = (stdin.lock(), BufWriter::new(stdout.lock()));

    let mut buf = String::new();
    stdin.read_to_string(&mut buf).unwrap();

    let mut lines = buf.lines();

    loop {
        let n = parse_int(lines.next().unwrap());

        if n == 0 {
            return;
        }

        let mut gcd_set = HashSet::<i32>::new();
        let mut gcd_accum = HashSet::new();

        for _ in 0..n {
            let num = parse_int(lines.next().unwrap());

            gcd_accum = gcd_accum.iter().map(|&g| gcd(g, num)).collect();
            gcd_accum.insert(num);

            gcd_set.extend(&gcd_accum);
        }

        writeln!(stdout, "{}", gcd_set.len()).unwrap();
    }
}

fn gcd(mut a: i32, mut b: i32) -> i32 {
    loop {
        (a, b) = (b, a % b);

        if b == 0 {
            return a;
        }
    }
}

fn parse_int(buf: &str) -> i32 {
    buf.parse().unwrap()
}
