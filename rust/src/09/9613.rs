use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i64>);
    let mut input = || input.next().unwrap();

    for _ in 0..input() {
        let nums: Vec<_> = (0..input()).map(|_| input()).collect();
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
