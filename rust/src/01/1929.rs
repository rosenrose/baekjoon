use std::io::{stdin, stdout, BufRead, BufWriter, Write};

fn main() {
    let (stdin, stdout) = (stdin(), stdout());
    let (mut stdin, mut stdout) = (stdin.lock(), BufWriter::new(stdout.lock()));

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();

    if let [m, n] = parse_int_vec(&buf)[..] {
        let prime_nums = get_prime_nums(n as usize);

        for prime_num in prime_nums.iter().filter(|&&p| p >= m) {
            writeln!(stdout, "{prime_num}").unwrap();
        }
    }
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

fn parse_int_vec(buf: &String) -> Vec<i32> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}
