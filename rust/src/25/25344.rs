use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());

    let period = get_lcm(input.skip(1));

    println!("{period}");
}

fn get_lcm<I>(nums: I) -> i32
where
    I: Iterator<Item = i32>,
{
    nums.reduce(|a, b| a / get_gcd(a, b) * b).unwrap()
}

fn get_gcd(mut a: i32, mut b: i32) -> i32 {
    loop {
        if b == 0 {
            return a;
        }

        (a, b) = (b, a % b);
    }
}
