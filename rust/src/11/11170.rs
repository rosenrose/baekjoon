use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>().unwrap());

    let sum_accum = (1..=1_000_000).fold(vec![0, 1], |mut acc, mut num| {
        let mut count = 0;

        while num > 0 {
            if num % 10 == 0 {
                count += 1;
            }

            num /= 10;
        }

        acc.push(*acc.last().unwrap() + count);
        acc
    });

    for _ in 0..input.next().unwrap() {
        let (n, m) = (input.next().unwrap(), input.next().unwrap());

        println!("{}", sum_accum[m + 1] - sum_accum[n]);
    }
}
