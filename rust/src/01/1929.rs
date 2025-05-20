use std::fmt::Write;
use std::io;

const MAX: usize = 1_000_000 + 1;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut output = String::new();

    let [m, n] = [(); 2].map(|_| input.next().unwrap());
    let prime_nums = get_prime_nums(n as usize);

    for prime_num in prime_nums.iter().filter(|&&p| p >= m) {
        writeln!(output, "{prime_num}").unwrap();
    }

    print!("{output}");
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
