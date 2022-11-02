use std::io::{stdout, BufWriter, Write};

fn main() {
    let stdout = stdout();
    let mut stdout = BufWriter::new(stdout.lock());

    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    if let [m, n] = parse_int_vec(&buf)[..] {
        let prime_sieve = get_prime_sieve(n);
        let prime_nums = (m..=n).filter(|&i| prime_sieve[i as usize]);

        for prime_num in prime_nums {
            writeln!(stdout, "{prime_num}").unwrap();
        }
    }
}

fn get_prime_sieve(num: i32) -> Vec<bool> {
    let mut prime_sieve = vec![true; (num + 1) as usize];
    prime_sieve[0] = false;
    prime_sieve[1] = false;

    for i in 2..=(num as f64).sqrt() as usize {
        if !prime_sieve[i] {
            continue;
        }

        for j in ((i * i)..=num as usize).step_by(i) {
            prime_sieve[j] = false;
        }
    }

    prime_sieve
}

fn parse_int_vec(buf: &String) -> Vec<i32> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}
