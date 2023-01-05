use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());

    let (n, x) = (input.next().unwrap(), input.next().unwrap());
    let mut order: Vec<_> = input
        .enumerate()
        .map(|(i, t)| {
            let mut shout = x + i as i32;

            while shout <= t {
                shout += n;
            }

            (shout, i + 1)
        })
        .collect();

    order.sort();

    println!("{}", order[0].1);
}
