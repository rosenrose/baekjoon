fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let mut n: i32 = buf.trim().parse().unwrap();
    let prime_nums = (2..=n).filter(|&i| is_prime(i));

    for prime_num in prime_nums {
        while n % prime_num == 0 {
            println!("{prime_num}");
            n /= prime_num;
        }

        if is_prime(n) {
            println!("{n}");
            return;
        }

        if n == 1 {
            return;
        }
    }
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
