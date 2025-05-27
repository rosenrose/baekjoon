use std::fmt::Write;
use std::io;

const MAX: usize = 1_000_000;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut output = String::new();

    let (prime_nums, sieve) = get_prime_nums(MAX);

    for n in input.skip(1) {
        let count = get_goldbach_partition_count(n, &prime_nums, &sieve);

        writeln!(output, "{count}").unwrap();
    }

    print!("{output}");
}

fn get_goldbach_partition_count(num: i32, prime_nums: &[i32], sieve: &[bool]) -> usize {
    if num == 4 {
        return 1;
    }

    prime_nums
        .iter()
        .take_while(|&&p| p <= num / 2)
        .filter(|&&p| sieve[(num - p) as usize])
        .count()
}

fn get_prime_nums(num: usize) -> (Vec<i32>, [bool; MAX + 1]) {
    let mut sieve = [true; MAX + 1];
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
