use std::io::{stdin, stdout, BufRead, BufWriter, Write};

fn main() {
    let stdin = stdin();
    let stdout = stdout();
    let mut stdin = stdin.lock();
    let mut stdout = BufWriter::new(stdout.lock());

    let mut buf = String::new();
    let mut nums = Vec::new();
    let mut max = 0;

    loop {
        buf.clear();
        stdin.read_line(&mut buf).unwrap();

        let n: i32 = buf.trim().parse().unwrap();

        if n == 0 {
            break;
        }

        nums.push(n);

        if n > max {
            max = n;
        }
    }

    let prime_sieve = get_prime_sieve(max * 2);

    for num in nums {
        let prime_nums = (num + 1..=num * 2).filter(|&i| prime_sieve[i as usize]);

        writeln!(stdout, "{}", prime_nums.count()).unwrap();
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
