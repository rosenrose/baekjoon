use std::io::{stdin, stdout, BufRead, BufWriter, Write};

fn main() {
    let stdin = stdin();
    let stdout = stdout();
    let mut stdin = stdin.lock();
    let mut stdout = BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();

    let t: i32 = buf.trim().parse().unwrap();

    for _ in 0..t {
        buf.clear();
        stdin.read_line(&mut buf).unwrap();

        let n: i32 = buf.trim().parse().unwrap();
        let (a, b) = get_goldbach_partition(n).unwrap();

        writeln!(stdout, "{a} {b}").unwrap();
    }
}

fn get_goldbach_partition(num: i32) -> Option<(i32, i32)> {
    if num == 4 {
        return Some((2, 2));
    }

    let half = num / 2;
    let half = if half % 2 == 0 { half - 1 } else { half };

    let prime_nums = (3..=half).rev().step_by(2).filter(|&n| is_prime(n));

    for a in prime_nums {
        let b = num - a;

        if is_prime(b) {
            return Some((a, b));
        }
    }

    None
}

fn is_prime(num: i32) -> bool {
    if num == 1 {
        return false;
    }
    if num == 2 {
        return true;
    }

    for n in 2..=(num as f64).sqrt() as i32 {
        if num % n == 0 {
            return false;
        }
    }

    true
}