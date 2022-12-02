fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    if let [_, s] = parse_int_vec(&buf)[..] {
        read_line(&mut buf);

        let diffs = buf
            .split_whitespace()
            .map(|num| num.parse::<i32>().unwrap().abs_diff(s));

        let gcd = get_gcd(diffs);

        println!("{gcd}");
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

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}

fn parse_int_vec(buf: &String) -> Vec<i32> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}
