fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let [a, b] = parse_int_vec(&buf)[..] else { return };
    let prime_nums = get_prime_nums(((b as f64).sqrt()) as usize);

    let count = prime_nums.iter().fold(0, |acc, &p| {
        acc + (b.ilog(p) - (a - 1).checked_ilog(p).unwrap_or(1))
    });

    println!("{count}");
}

fn get_prime_nums(num: usize) -> Vec<i64> {
    let mut sieve = vec![true; num + 1];
    let mut prime_nums = Vec::new();

    for i in 2..=num {
        if sieve[i] {
            prime_nums.push(i as i64);
        }

        for &p in prime_nums.iter().take_while(|&&p| i * p as usize <= num) {
            sieve[i * p as usize] = false;

            if i as i64 % p == 0 {
                break;
            }
        }
    }

    prime_nums
}

fn parse_int_vec(buf: &str) -> Vec<i64> {
    buf.split_whitespace().flat_map(str::parse).collect()
}
