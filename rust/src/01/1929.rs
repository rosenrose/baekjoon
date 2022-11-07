use std::io::{stdin, stdout, BufRead, BufWriter, Write};

fn main() {
    let (stdin, stdout) = (stdin(), stdout());
    let (mut stdin, mut stdout) = (stdin.lock(), BufWriter::new(stdout.lock()));

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();

    if let [m, n] = parse_int_vec(&buf)[..] {
        let prime_sieve = get_prime_sieve(n as usize);
        let prime_nums = (m..=n).filter(|&i| prime_sieve[i as usize]);

        for prime_num in prime_nums {
            writeln!(stdout, "{prime_num}").unwrap();
        }
    }
}

fn get_prime_sieve(num: usize) -> Vec<bool> {
    let mut prime_sieve = vec![true; num + 1];
    prime_sieve[0] = false;
    prime_sieve[1] = false;

    (2..).take_while(|i| i * i <= num).for_each(|i| {
        if !prime_sieve[i] {
            return;
        }

        for j in ((i * i)..=num).step_by(i) {
            prime_sieve[j] = false;
        }
    });

    prime_sieve
}

fn parse_int_vec(buf: &String) -> Vec<i32> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}
