use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i64>().unwrap());

    let lcm1 = get_lcm(input.next().unwrap(), input.next().unwrap());
    let lcm2 = input.next().unwrap();

    let mut divisors = (1..)
        .take_while(|i| i * i <= lcm2)
        .fold(Vec::new(), |mut acc, i| {
            if lcm2 % i == 0 {
                acc.push(i);
                acc.push(lcm2 / i);
            }

            acc
        });

    divisors.dedup();
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
