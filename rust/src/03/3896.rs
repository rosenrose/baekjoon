use std::io::{stdin, stdout, BufWriter, Read, Write};

fn main() {
    let (stdin, stdout) = (stdin(), stdout());
    let (mut stdin, mut stdout) = (stdin.lock(), BufWriter::new(stdout.lock()));

    let mut buf = String::new();
    stdin.read_to_string(&mut buf).unwrap();

    for line in buf.lines().skip(1) {
        let k: i32 = line.parse().unwrap();

        if is_prime(k) {
            writeln!(stdout, "0").unwrap();
            continue;
        }

        let down = (2..k).rev().filter(|&i| is_prime(i)).next().unwrap();
        let up = (k + 1..).filter(|&i| is_prime(i)).next().unwrap();

        writeln!(stdout, "{}", up - down).unwrap();
    }
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
