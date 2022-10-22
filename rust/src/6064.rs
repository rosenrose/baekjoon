use std::io::{stdin, stdout, BufRead, BufWriter, Write};

fn main() {
    let stdin = stdin();
    let stdout = stdout();
    let mut stdin = stdin.lock();
    let mut stdout = BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();

    let t: i32 = buf.trim().parse().unwrap();

    'outer: for _ in 0..t {
        buf.clear();
        stdin.read_line(&mut buf).unwrap();

        if let [m, n, x, y] = parse_int_vec(&buf)[..] {
            let total_years = get_lcm(m, n);

            let (start_year, compare_year) = if m > n {
                (x, if y == n { 0 } else { y })
            } else {
                (y, if x == m { 0 } else { x })
            };
            let (multi, div) = (m.max(n), m.min(n));

            for i in (start_year..=total_years).step_by(multi) {
                if i % div == compare_year {
                    writeln!(stdout, "{i}").unwrap();
                    continue 'outer;
                }
            }

            writeln!(stdout, "-1").unwrap();
        }
    }
}

fn get_lcm(a: usize, b: usize) -> usize {
    (a * b) / get_gcd(a, b)
}

fn get_gcd(mut a: usize, mut b: usize) -> usize {
    if a < b {
        (a, b) = (b, a);
    }

    loop {
        (a, b) = (b, a % b);

        if b == 0 {
            return a;
        }
    }
}

fn parse_int_vec(buf: &String) -> Vec<usize> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}
