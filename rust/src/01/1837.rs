struct BigInt(Vec<i32>);

impl BigInt {
    fn parse(input: &str) -> Self {
        Self(
            input
                .as_bytes()
                .rchunks(3)
                .map(|chunk| {
                    let mut exp = 1;

                    chunk.iter().rev().fold(0, |acc, &ch| {
                        let num = (ch as i32 - '0' as i32) * exp;
                        exp *= 10;

                        acc + num
                    })
                })
                .collect(),
        )
    }

    fn rem(&self, m: i32) -> i32 {
        let mut exp_rem = 1;

        self.0.iter().fold(0, |acc, num| {
            let rem = (num * exp_rem) % m;

            for _ in 0..3 {
                exp_rem = (exp_rem * 10) % m;
            }

            (acc + rem) % m
        })
    }
}

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let mut input = buf.split_whitespace();
    let p = BigInt::parse(input.next().unwrap());
    let k: usize = input.next().unwrap().parse().unwrap();

    for prime in get_prime_nums(k - 1) {
        if p.rem(prime) == 0 {
            println!("BAD {prime}");
            return;
        }
    }

    println!("GOOD");
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
