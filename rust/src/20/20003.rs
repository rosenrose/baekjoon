use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i64>().unwrap());
    let mut input = || input.next().unwrap();

    let n = input();
    let (coin_a, coin_b) = (0..n - 1).fold((input(), input()), |acc, _| {
        let (coin_a, coin_b) = acc;
        let (a, b) = (input(), input());

        let lcm = get_lcm(coin_b, b);
        let gcd = get_gcd(coin_a * (lcm / coin_b), a * (lcm / b));

        (gcd, lcm)
    });

    let gcd = get_gcd(coin_a, coin_b);

    println!("{} {}", coin_a / gcd, coin_b / gcd);
}

fn get_lcm(a: i64, b: i64) -> i64 {
    a / get_gcd(a, b) * b
}

fn get_gcd(mut a: i64, mut b: i64) -> i64 {
    loop {
        if b == 0 {
            return a;
        }

        (a, b) = (b, a % b);
    }
}
