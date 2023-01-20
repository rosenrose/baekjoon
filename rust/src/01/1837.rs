fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let mut nums = buf.split_whitespace();
    let p: Vec<_> = nums
        .next()
        .unwrap()
        .as_bytes()
        .rchunks(3)
        .map(|chunk| {
            let mut exp = 1;

            chunk.iter().rev().fold(0, |acc, ch| {
                let num = (ch - '0' as u8) as i32 * exp;
                exp *= 10;

                acc + num
            })
        })
        .collect();

    let k: usize = nums.next().unwrap().parse().unwrap();

    for prime in get_prime_nums(k - 1) {
        if bigint_rem(&p, prime) == 0 {
            println!("BAD {prime}");
            return;
        }
    }

    println!("GOOD");
}

fn bigint_rem(bigint: &Vec<i32>, m: i32) -> i32 {
    let mut exp_rem = 1;

    bigint.iter().fold(0, |acc, num| {
        let rem = ((num % m) * exp_rem) % m;

        for _ in 0..3 {
            exp_rem = (exp_rem * (10 % m)) % m;
        }

        (acc + rem) % m
    })
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
