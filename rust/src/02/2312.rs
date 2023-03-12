use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.lines().flat_map(str::parse::<i32>);
    let mut output = String::new();

    for mut n in input.skip(1) {
        let prime_nums = (2..=n).filter(|&i| is_prime(i));
        let mut counts = [0_u8; 100_000];
        let mut max = 0;

        for p in prime_nums {
            while n % p == 0 {
                counts[p as usize] += 1;
                n /= p;
                max = p;
            }

            if is_prime(n) {
                counts[n as usize] += 1;
                max = n;
                break;
            }

            if n == 1 {
                break;
            }
        }

        for p in (2..=max).filter(|&p| counts[p as usize] > 0) {
            writeln!(output, "{p} {}", counts[p as usize]).unwrap();
        }
    }

    print!("{output}");
}

fn is_prime(num: i32) -> bool {
    if num <= 1 {
        return false;
    }

    (2..).take_while(|i| i * i <= num).all(|i| num % i != 0)
}
