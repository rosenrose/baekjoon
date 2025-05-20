struct BigInt(Vec<i32>);

const MAX: usize = 1_000_000 + 1;
const DIGITS: usize = 3;

impl BigInt {
    fn parse(input: &str) -> Self {
        Self(
            input
                .as_bytes()
                .rchunks(DIGITS)
                .map(|chunk| {
                    chunk
                        .iter()
                        .fold(0, |acc, ch| acc * 10 + (ch - b'0') as i32)
                })
                .collect(),
        )
    }

    fn rem(&self, m: i32) -> i32 {
        let mut pow_rem = 1;

        self.0.iter().fold(0, |acc, num| {
            let rem = (num * pow_rem) % m;

            for _ in 0..DIGITS {
                pow_rem = (pow_rem * 10) % m;
            }

            (acc + rem) % m
        })
    }
}

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let (p, k) = buf.trim().split_once(' ').unwrap();
    let p = BigInt::parse(p);
    let k: usize = k.parse().unwrap();

    for prime in get_prime_nums(k - 1) {
        if p.rem(prime) == 0 {
            println!("BAD {prime}");
            return;
        }
    }

    println!("GOOD");
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
