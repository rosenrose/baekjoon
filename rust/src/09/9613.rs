use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i64>().unwrap());

    for _ in 0..input.next().unwrap() {
        let n = input.next().unwrap();
        let nums: Vec<_> = (0..n).map(|_| input.next().unwrap()).collect();

        let mut sum = 0;

        for i in 0..nums.len() - 1 {
            for j in i + 1..nums.len() {
                sum += get_gcd(nums[i], nums[j]);
            }
        }

        println!("{sum}");
    }
}

fn get_gcd(mut a: i64, mut b: i64) -> i64 {
    loop {
        if b == 0 {
            return a;
        }

        (a, b) = (b, a % b);
    }
}
