use std::io::{stdin, stdout, BufRead, BufWriter, Write};

fn main() {
    let (stdin, stdout) = (stdin(), stdout());
    let (mut stdin, mut stdout) = (stdin.lock(), BufWriter::new(stdout.lock()));

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
        max = n.max(max);
    }

    let prime_sieve = get_prime_sieve((max * 2) as usize);

    for num in nums {
        let prime_nums = (num + 1..=num * 2).filter(|&i| prime_sieve[i as usize]);

        writeln!(stdout, "{}", prime_nums.count()).unwrap();
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
