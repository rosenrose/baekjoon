use std::io;

const MAX: usize = 10_000_000 + 1;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i64>);

    let [a, b] = [(); 2].map(|_| input.next().unwrap());
    let prime_nums = get_prime_nums(((b as f64).sqrt()) as usize);

    let count: u32 = prime_nums
        .iter()
        .map(|&p| {
            b.ilog(p as i64).saturating_sub(1)
                - (a - 1)
                    .checked_ilog(p as i64)
                    .unwrap_or(0)
                    .saturating_sub(1)
        })
        .sum();

    println!("{count}");
}

fn get_prime_nums(num: usize) -> Vec<i32> {
    let mut sieve = [true; MAX];
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
