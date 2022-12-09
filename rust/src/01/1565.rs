use std::collections::HashSet;
use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i64>().unwrap());

    let n = input.next().unwrap();
    input.next();

    let d: Vec<_> = (0..n).map(|_| input.next().unwrap()).collect();
    let m: Vec<_> = input.collect();

    let gcd = get_gcd(m.into_iter());
    let mut lcm = 1;

    for num in d {
        lcm = get_lcm(lcm, num);

        if lcm > gcd {
            println!("0");
            return;
        }
    }

    let count = (1..)
        .take_while(|i| i * i <= gcd)
        .fold(HashSet::new(), |mut acc, i| {
            if gcd % i != 0 {
                return acc;
            }

            if i % lcm == 0 {
                acc.insert(i);
            }
            if (gcd / i) % lcm == 0 {
                acc.insert(gcd / i);
            }

            acc
        })
        .len();

    println!("{count}");
}

fn get_lcm(a: i64, b: i64) -> i64 {
    a / get_gcd([a, b].into_iter()) * b
}

fn get_gcd<I>(nums: I) -> i64
where
    I: Iterator<Item = i64>,
{
    nums.reduce(|mut a, mut b| loop {
        if b == 0 {
            return a;
        }

        (a, b) = (b, a % b);
    })
    .unwrap()
}
