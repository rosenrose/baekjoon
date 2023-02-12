use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut input = || input.next().unwrap();
    const INF: &str = "INFINITY";

    for _ in 0..input() {
        let n = input();

        if n == 1 {
            println!("{INF}");
            continue;
        }

        let nums: Vec<_> = (0..n).map(|_| input()).collect();
        let diffs = (1..nums.len()).map(|i| nums[i].abs_diff(nums[i - 1]));
        let gcd = get_gcd(diffs);

        if gcd == 0 {
            println!("{INF}");
            continue;
        }

        println!("{gcd}");
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
