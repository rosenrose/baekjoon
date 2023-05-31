use std::collections::VecDeque;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut input = || input.next().unwrap();

    const MAX: usize = 9999;
    let prime_nums = get_prime_nums(MAX);
    let mut prime_digits = [[0; 4]; MAX + 1];

    for &p in prime_nums.iter().filter(|&&p| p >= 1000) {
        prime_digits[p] = to_digits(p);
    }

    'outer: for (a, b) in (0..input()).map(|_| (input(), input())) {
        let mut visited = [false; MAX + 1];
        visited[a] = true;

        let mut queue = VecDeque::from([(a, 0)]);

        while let Some((num, step)) = queue.pop_front() {
            if num == b {
                println!("{step}");
                continue 'outer;
            }

            let digits = to_digits(num);
            let adjacents = prime_nums.iter().filter(|&&p| {
                prime_digits[p]
                    .iter()
                    .zip(&digits)
                    .filter(|(x, y)| x != y)
                    .count()
                    == 1
            });

            for &adj in adjacents {
                if visited[adj] {
                    continue;
                }

                visited[adj] = true;
                queue.push_back((adj, step + 1));
            }
        }

        println!("Impossible");
    }
}

fn get_prime_nums(num: usize) -> Vec<usize> {
    let mut sieve = vec![true; num + 1];
    let mut prime_nums = Vec::new();

    for i in 2..=num {
        if sieve[i] {
            prime_nums.push(i);
        }

        for &p in prime_nums.iter().take_while(|&&p| i * p <= num) {
            sieve[i * p] = false;

            if i % p == 0 {
                break;
            }
        }
    }

    prime_nums
}

fn to_digits(mut num: usize) -> [u8; 4] {
    let mut digits = [0; 4];

    for digit in digits.iter_mut().rev() {
        *digit = (num % 10) as u8;
        num /= 10;
    }

    digits
}
