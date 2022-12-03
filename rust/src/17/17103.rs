use std::io::{stdin, stdout, BufWriter, Read, Write};

fn main() {
    let (stdin, stdout) = (stdin(), stdout());
    let (mut stdin, mut stdout) = (stdin.lock(), BufWriter::new(stdout.lock()));

    let mut buf = String::new();
    stdin.read_to_string(&mut buf).unwrap();

    let (prime_nums, sieve) = get_prime_nums(1_000_000);

    buf.lines().skip(1).for_each(|line| {
        let num: i32 = line.parse().unwrap();
        let count = get_goldbach_partition_count(num, &prime_nums, &sieve);

        writeln!(stdout, "{count}").unwrap();
    });
}

fn get_goldbach_partition_count(num: i32, prime_nums: &Vec<i32>, sieve: &Vec<bool>) -> usize {
    if num == 4 {
        return 1;
    }

    prime_nums
        .iter()
        .take_while(|&&p| p <= num / 2)
        .filter(|&&p| sieve[(num - p) as usize])
        .count()
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
