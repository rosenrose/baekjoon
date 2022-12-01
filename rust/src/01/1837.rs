fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let mut nums = buf.split_whitespace();
    let p: Vec<_> = nums
        .next()
        .unwrap()
        .chars()
        .rev()
        .map(|c| c.to_digit(10).unwrap() as i32)
        .collect();

    let k: i32 = nums.next().unwrap().parse().unwrap();

    for prime in get_prime_nums((k - 1) as usize) {
        if bigint_mod(&p, prime) == 0 {
            println!("BAD {prime}");
            return;
        }
    }

    println!("GOOD");
}

fn bigint_mod(bigint: &Vec<i32>, m: i32) -> i32 {
    let mut exp_rem = 1 % m;

    bigint.iter().fold(0, |acc, num| {
        let rem = (num % m * exp_rem % m) % m;
        exp_rem = (exp_rem % m * 10 % m) % m;

        (acc % m + rem % m) % m
    })
}

fn get_prime_nums(num: usize) -> Vec<i32> {
    let mut sieve = vec![true; num + 1];
    (sieve[0], sieve[1]) = (false, false);

    for i in (2..).take_while(|i| i * i <= num) {
        if !sieve[i] {
            continue;
        }

        for j in (i * i..=num).step_by(i) {
            sieve[j] = false;
        }
    }

    sieve
        .iter()
        .enumerate()
        .filter_map(|(i, &s)| if s { Some(i as i32) } else { None })
        .collect()
}
