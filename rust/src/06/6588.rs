use std::io::{stdin, stdout, BufRead, BufWriter, Write};

fn main() {
    let (stdin, stdout) = (stdin(), stdout());
    let (mut stdin, mut stdout) = (stdin.lock(), BufWriter::new(stdout.lock()));

    let mut buf = String::new();

    loop {
        buf.clear();
        stdin.read_line(&mut buf).unwrap();

        let n: i32 = buf.trim().parse().unwrap();

        if n == 0 {
            return;
        }

        match get_goldbach_partition(n) {
            Some((a, b)) => writeln!(stdout, "{n} = {a} + {b}").unwrap(),
            None => writeln!(stdout, "Goldbach's conjecture is wrong.").unwrap(),
        };
    }
}

fn get_goldbach_partition(num: i32) -> Option<(i32, i32)> {
    if num == 4 {
        return Some((2, 2));
    }

    let half = num / 2;
    let half = if half % 2 == 0 { half - 1 } else { half };

    let prime_nums = (3..=half).step_by(2).filter(|&n| is_prime(n));

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

    for i in (2..).take_while(|i| i * i <= num) {
        if num % i == 0 {
            return false;
        }
    }

    true
}
