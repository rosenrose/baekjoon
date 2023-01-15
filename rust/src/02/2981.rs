use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.lines().flat_map(str::parse::<i32>);

    let nums: Vec<_> = input.skip(1).collect();
    let diffs = (1..nums.len()).map(|i| nums[i].abs_diff(nums[i - 1]));
    let gcd = get_gcd(diffs);

    let mut divisors = (1..)
        .take_while(|i| i * i <= gcd)
        .fold(Vec::new(), |mut acc, i| {
            if gcd % i == 0 {
                acc.push(i);
                acc.push(gcd / i);
            }

            acc
        });

    divisors.dedup();
    divisors.sort();

    for num in divisors.iter().skip(1) {
        print!("{num} ");
    }
}

fn get_gcd<I>(nums: I) -> u32
where
    I: Iterator<Item = u32>,
{
    nums.reduce(|mut a, mut b| loop {
        if b == 0 {
            return a;
        }

        (a, b) = (b, a % b);
    })
    .unwrap()
}
