use std::fmt::Write;
use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());
    let mut output = String::new();

    let (prime_nums, sieve) = get_prime_nums(10000);

    for n in input.skip(1) {
        let (a, b) = get_goldbach_partition(n, &prime_nums, &sieve).unwrap();

        writeln!(output, "{a} {b}").unwrap();
    }

    print!("{output}");
}

fn get_goldbach_partition(
    num: i32,
    prime_nums: &Vec<i32>,
    sieve: &Vec<bool>,
) -> Option<(i32, i32)> {
    if num == 4 {
        return Some((2, 2));
    }

    for &p in prime_nums.iter().skip_while(|&&p| p < num / 2) {
        if sieve[(num - p) as usize] {
            return Some((num - p, p));
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
