use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());

    let max_prize = (0..input.next().unwrap())
        .map(|_| {
            let nums: Vec<_> = (0..4).map(|_| input.next().unwrap()).collect();
            let counts: Vec<_> = nums
                .iter()
                .map(|num| {
                    let count = nums.iter().filter(|&n| n == num).count();
                    (num, count)
                })
                .collect();

            if counts.iter().all(|&(_, c)| c == 4) {
                return 50000 + nums[0] * 5000;
            }

            if counts.iter().any(|&(_, c)| c == 3) {
                let (a, _) = *counts.iter().max_by_key(|(_, c)| c).unwrap();

                return 10000 + a * 1000;
            }

            if counts.iter().all(|&(_, c)| c == 2) {
                let a = nums[0];
                let b = nums.iter().find(|&&num| num != a).unwrap();

                return 2000 + a * 500 + b * 500;
            }

            if counts.iter().any(|&(_, c)| c == 2) {
                let (a, _) = *counts.iter().max_by_key(|(_, c)| c).unwrap();

                return 1000 + a * 100;
            }

            nums.iter().max().unwrap() * 100
        })
        .max()
        .unwrap();

    println!("{max_prize}");
}
