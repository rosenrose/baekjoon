use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i64>);
    let mut input = || input.next().unwrap();

    let lcm1 = get_lcm(input(), input());
    let lcm2 = input();

    let mut divisors = Vec::new();

    for i in (1..).take_while(|i| i * i <= lcm2) {
        if lcm2 % i != 0 {
            continue;
        }

        divisors.push(i);

        if i != lcm2 / i {
            divisors.push(lcm2 / i);
        }
    }

    divisors.sort();

    for c in divisors {
        if get_lcm(lcm1, c) == lcm2 {
            println!("{c}");
            return;
        }
    }

    println!("-1");
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
