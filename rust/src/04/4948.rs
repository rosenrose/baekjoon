use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut output = String::new();

    let prime_nums = get_prime_nums(123_456 * 2);

    for num in input.take_while(|&num| num != 0) {
        let count = prime_nums
            .iter()
            .skip_while(|&&p| p <= num)
            .take_while(|&&p| p <= 2 * num)
            .count();

        writeln!(output, "{count}").unwrap();
    }

    print!("{output}");
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
