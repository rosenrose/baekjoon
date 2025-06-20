use std::io;

const MAX: usize = 100_000;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines().flat_map(str::parse::<u32>);

    let n = input.next().unwrap() as usize;
    let mut nums = [0; MAX];

    for (i, num) in input.enumerate() {
        nums[i] = num;
    }

    let mut gcd = nums[0].abs_diff(nums[1]);

    for i in 2..n {
        gcd = get_gcd(nums[i - 1].abs_diff(nums[i]), gcd);
    }

    let mut count = 0;

    for i in 1..n {
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
