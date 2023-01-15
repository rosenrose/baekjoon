use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

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
