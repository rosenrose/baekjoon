use std::io::{stdin, stdout, BufRead, BufWriter, Write};

fn main() {
    let (stdin, stdout) = (stdin(), stdout());
    let (mut stdin, mut stdout) = (stdin.lock(), BufWriter::new(stdout.lock()));

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();

    let t: i32 = buf.trim().parse().unwrap();

    for _ in 0..t {
        buf.clear();
        stdin.read_line(&mut buf).unwrap();

        if let [m, n, x, y] = parse_int_vec(&buf)[..] {
            let lcm = get_lcm(m, n);
            let (gcd, mut a, mut b) = get_ex_gcd(m, n);
            // println!("{lcm} {gcd} {a} {b]");
            if (y - x) % gcd != 0 {
                writeln!(stdout, "-1").unwrap();
                continue;
            }

            let multiple = (y - x) / gcd;

            a *= multiple;
            a %= n / gcd;

            b *= -multiple;
            b %= m / gcd;

            let mut left = m * a + x;
            let mut right = n * b + y;
            while left <= 0 {
                left += lcm;
            }
            while right <= 0 {
                right += lcm;
            }
            assert_eq!(left, right);
            // println!("{left} {right}");

            writeln!(stdout, "{left}").unwrap();
        }
    }
}

fn get_lcm(a: i32, b: i32) -> i32 {
    (a * b) / get_gcd(a, b)
}

fn get_gcd(mut a: i32, mut b: i32) -> i32 {
    loop {
        (a, b) = (b, a % b);

        if b == 0 {
            return a;
        }
    }
}

fn get_ex_gcd(a: i32, b: i32) -> (i32, i32, i32) {
    let (mut r1, mut r2) = (a, b);
    let (mut s1, mut s2) = (1, 0);
    let (mut t1, mut t2) = (0, 1);
    let mut q;

    loop {
        q = r1 / r2;
        (r1, r2) = (r2, r1 % r2);

        (s1, s2) = (s2, s1 - s2 * q);
        (t1, t2) = (t2, t1 - t2 * q);

        if r2 == 0 {
            // if s1 < 0 {
            //     s1 += b;
            //     t1 -= a;
            // }
            return (r1, s1, t1);
        }
    }
}

fn parse_int_vec(buf: &String) -> Vec<i32> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}

/* use std::io::{stdin, stdout, BufRead, BufWriter, Write};

fn main() {
    let (stdin, stdout) = (stdin(), stdout());
    let (mut stdin, mut stdout) = (stdin.lock(), BufWriter::new(stdout.lock()));

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
            let (mod1, mod2) = (m.max(n), m.min(n));

            for year in (start_year..=total_years).step_by(mod1) {
                if year % mod2 == compare_year {
                    writeln!(stdout, "{year}").unwrap();
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
 */
