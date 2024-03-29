use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.lines().flat_map(str::parse::<i32>);

    let nums: Vec<_> = input.skip(1).collect();
    let diffs = (1..nums.len()).map(|i| nums[i].abs_diff(nums[i - 1]));
    let gcd = get_gcd(diffs);

    let mut divisors = Vec::new();

    for i in (1..).take_while(|i| i * i <= gcd) {
        if gcd % i != 0 {
            continue;
        }

        divisors.push(i);

        if i != gcd / i {
            divisors.push(gcd / i);
        }
    }

    divisors.sort();

    for num in &divisors[1..] {
        print!("{num} ");
    }
}

fn get_gcd(nums: impl Iterator<Item = u32>) -> u32 {
    nums.reduce(|mut a, mut b| loop {
        if b == 0 {
            return a;
        }

        (a, b) = (b, a % b);
    })
    .unwrap()
}
