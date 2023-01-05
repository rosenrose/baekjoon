use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());

    let gcd = get_gcd(input.skip(1));
    let mut divisors = (1..)
        .take_while(|i| i * i <= gcd)
        .fold(Vec::new(), |mut acc, i| {
            if gcd % i == 0 {
                acc.push(i);
                acc.push(gcd / i);
            }

            acc
        });

    divisors.dedup();
    divisors.sort();

    for divisor in divisors {
        println!("{divisor}");
    }
}

fn get_gcd<I>(nums: I) -> i32
where
    I: Iterator<Item = i32>,
{
    nums.reduce(|mut a, mut b| loop {
        if b == 0 {
            return a;
        }

        (a, b) = (b, a % b);
    })
    .unwrap()
}
