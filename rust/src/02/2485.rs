use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.lines().flat_map(str::parse::<u32>);

    let nums: Vec<_> = input.skip(1).collect();
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
