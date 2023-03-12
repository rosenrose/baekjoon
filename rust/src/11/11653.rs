fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let mut n: i32 = buf.trim().parse().unwrap();
    let prime_nums = (2..=n).filter(|&i| is_prime(i));

    for p in prime_nums {
        while n % p == 0 {
            println!("{p}");
            n /= p;
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
    if num <= 1 {
        return false;
    }

    (2..).take_while(|i| i * i <= num).all(|i| num % i != 0)
}
