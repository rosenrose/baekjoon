use std::io::{stdin, Read};

fn main() {
    let stdin = stdin();
    let mut stdin = stdin.lock();

    let mut buf = String::new();
    stdin.read_to_string(&mut buf).unwrap();

    let nums: Vec<_> = buf
        .split_ascii_whitespace()
        .skip(1)
        .map(|s| s.parse::<u32>().unwrap())
        .collect();

    let mut gcd = nums[0].abs_diff(nums[1]);

    for i in 2..nums.len() {
        gcd = get_gcd(nums[i - 1].abs_diff(nums[i]), gcd);
    }

    let mut count = 0;

    for i in 1..nums.len() {
        let gap = nums[i - 1].abs_diff(nums[i]);

        count += (gap / gcd) - 1;
    }

    println!("{count}");
}

fn get_gcd(mut a: u32, mut b: u32) -> u32 {
    loop {
        if b == 0 {
            return a;
        }

        (a, b) = (b, a % b);
    }
}
