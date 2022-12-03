use std::io::{stdin, stdout, BufWriter, Read, Write};

fn main() {
    let (stdin, stdout) = (stdin(), stdout());
    let (mut stdin, mut stdout) = (stdin.lock(), BufWriter::new(stdout.lock()));

    let mut buf = String::new();
    stdin.read_to_string(&mut buf).unwrap();

    let (prime_nums, sieve) = get_prime_nums(1_000_000);

    buf.lines()
        .map(|s| s.parse::<i32>().unwrap())
        .for_each(|n| {
            if n == 0 {
                return;
            }

            match get_goldbach_partition(n, &prime_nums, &sieve) {
                Some((a, b)) => writeln!(stdout, "{n} = {a} + {b}").unwrap(),
                None => writeln!(stdout, "Goldbach's conjecture is wrong.").unwrap(),
            };
        });
}

fn get_goldbach_partition(
    num: i32,
    prime_nums: &Vec<i32>,
    sieve: &Vec<bool>,
) -> Option<(i32, i32)> {
    if num == 4 {
        return Some((2, 2));
    }

    for &p in prime_nums.iter().take_while(|&&p| p <= num / 2) {
        if sieve[(num - p) as usize] {
            return Some((p, num - p));
        }
    }

    None
}

fn get_prime_nums(num: usize) -> (Vec<i32>, Vec<bool>) {
    let mut sieve = vec![true; num + 1];
    (sieve[0], sieve[1]) = (false, false);

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

    (prime_nums, sieve)
}
