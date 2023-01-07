use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let prime_nums = buf
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse::<i32>().unwrap())
        .filter(|&n| is_prime(n));

    println!("{}", prime_nums.count());
}

fn is_prime(num: i32) -> bool {
    if num == 1 {
        return false;
    }

    for i in (2..).take_while(|i| i * i <= num) {
        if num % i == 0 {
            return false;
        }
    }

    true
}
