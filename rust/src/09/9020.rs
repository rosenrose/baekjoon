use std::fmt::Write;
use std::io;

const MAX: usize = 10000;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.lines().flat_map(str::parse::<i32>);
    let mut output = String::new();

    let (prime_nums, sieve) = get_prime_nums(MAX);

    for n in input.skip(1) {
        let (a, b) = get_goldbach_partition(n, &prime_nums, &sieve).unwrap();

        writeln!(output, "{a} {b}").unwrap();
    }

    print!("{output}");
}

fn get_goldbach_partition(num: i32, prime_nums: &[i32], sieve: &[bool]) -> Option<(i32, i32)> {
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
