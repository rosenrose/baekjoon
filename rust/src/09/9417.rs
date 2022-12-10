use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    for input in buf.lines().skip(1) {
        let nums: Vec<_> = input
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();

        let mut max_gcd = 0;

        for i in 0..nums.len() - 1 {
            for j in i + 1..nums.len() {
                max_gcd = get_gcd(nums[i], nums[j]).max(max_gcd);
            }
        }

        println!("{max_gcd}");
    }
}

fn get_gcd(mut a: i32, mut b: i32) -> i32 {
    loop {
        if b == 0 {
            return a;
        }

        (a, b) = (b, a % b);
    }
}
