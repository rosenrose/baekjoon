use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines().map(|s| s.parse::<i32>().unwrap());

    let (m, n) = (input.next().unwrap(), input.next().unwrap());
    let prime_nums: Vec<_> = (m..=n).filter(|&n| is_prime(n)).collect();

    if prime_nums.is_empty() {
        println!("{}", -1);
        return;
    }

    println!(
        "{}\n{}",
        prime_nums.iter().sum::<i32>(),
        prime_nums.iter().min().unwrap()
    );
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
