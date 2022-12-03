fn main() {
    let mut buf = String::new();
    read_line(&mut buf);
    read_line(&mut buf);

    let periods = parse_int_vec(&buf);
    let total_period = periods
        .windows(3)
        .fold(1, |lcm, chunk| get_lcm(chunk.iter().copied().chain([lcm])));

    println!("{total_period}");
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

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}

fn parse_int_vec(buf: &String) -> Vec<i32> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}
