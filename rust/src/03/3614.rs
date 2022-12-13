fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let n: i32 = buf.trim().parse().unwrap();
    let mut count = 0;

    for i in (1..).take_while(|i| i * i <= n) {
        if n % i != 0 {
            continue;
        }
        // a+b = i + 1 , a+b = n/i + 1
        count += (1..=(i + 1) / 2)
            .filter(|&a| get_gcd(a, i + 1 - a) == 1)
            .count();

        if i == n / i {
            continue;
        }

        count += (1..=(n / i + 1) / 2)
            .filter(|&a| get_gcd(a, n / i + 1 - a) == 1)
            .count();
    }

    println!("{count}");
}

fn get_gcd(mut a: i32, mut b: i32) -> i32 {
    loop {
        if b == 0 {
            return a;
        }

        (a, b) = (b, a % b);
    }
}
