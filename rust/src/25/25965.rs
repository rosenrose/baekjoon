use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i64>().unwrap());

    for _ in 0..input.next().unwrap() {
        let m = input.next().unwrap();
        let rewards: Vec<Vec<_>> = (0..m)
            .map(|_| (0..3).map(|_| input.next().unwrap()).collect())
            .collect();

        let (k, d, a) = (
            input.next().unwrap(),
            input.next().unwrap(),
            input.next().unwrap(),
        );

        let donation: i64 = rewards
            .iter()
            .map(|reward| (k * reward[0] - d * reward[1] + a * reward[2]).max(0))
            .sum();

        println!("{donation}");
    }
}
