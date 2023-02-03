use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let (_, s) = (input.next(), input.next().unwrap());
    let diffs = input.map(|num| num.abs_diff(s));
    let gcd = get_gcd(diffs);

    println!("{gcd}");
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
