use std::io;

const MAX_PRIME: usize = 1_003_001;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();

    let n: i32 = buf.trim().parse().unwrap();
    let prime_nums = get_prime_nums(MAX_PRIME);

    let prime_palindrome = prime_nums
        .iter()
        .skip_while(|&&p| p < n)
        .find(|p| is_palindrome(p.to_string().as_bytes()))
        .unwrap();

    println!("{prime_palindrome}");
}

fn get_prime_nums(num: usize) -> Vec<i32> {
    let mut sieve = [true; MAX_PRIME + 1];
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

fn is_palindrome(word: &[u8]) -> bool {
    let len = word.len();

    if len <= 1 {
        return true;
    }

    if word[0] != word[len - 1] {
        return false;
    }

    is_palindrome(&word[1..len - 1])
}
