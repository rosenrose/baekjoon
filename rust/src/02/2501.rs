use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>().unwrap());

    let (n, k) = (input.next().unwrap(), input.next().unwrap());
    let mut divisors = (1..)
        .take_while(|i| i * i <= n)
        .fold(Vec::new(), |mut acc, i| {
            if n % i == 0 {
                acc.push(i);
                acc.push(n / i);
            }

            acc
        });

    divisors.dedup();
    divisors.sort();

    println!("{}", divisors.get(k - 1).unwrap_or(&0));
}
