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

    let prime_nums = get_prime_nums((max * 2) as usize);

    for num in nums {
        let count = prime_nums
            .iter()
            .skip_while(|&&p| p <= num)
            .take_while(|&&p| p <= 2 * num)
            .count();

        writeln!(stdout, "{count}").unwrap();
    }
}

fn get_prime_nums(num: usize) -> Vec<i32> {
    let mut sieve = vec![true; num + 1];
    let mut prime_nums = Vec::new();

    for i in 2..=num {
        if sieve[i] {
            prime_nums.push(i as i32);
        }

        for &p in prime_nums.iter().take_while(|&&p| i * p as usize <= num) {
            sieve[i * p as usize] = false;

            if i as i32 % p == 0 {
                break;
            }
        }
    }

    prime_nums
}
