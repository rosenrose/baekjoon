use std::collections::HashSet;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i64>);
    let mut input = || input.next().unwrap();

    let (d_len, m_len) = (input(), input());
    let d: Vec<_> = (0..d_len).map(|_| input()).collect();
    let m: Vec<_> = (0..m_len).map(|_| input()).collect();

    let gcd = get_gcd(m.into_iter());
    let mut lcm = 1;

    for num in d {
        lcm = get_lcm(lcm, num);

        if lcm > gcd {
            println!("0");
            return;
        }
    }

    let count = (1..)
        .take_while(|i| i * i <= gcd)
        .fold(HashSet::new(), |mut acc, i| {
            if gcd % i != 0 {
                return acc;
            }

            if i % lcm == 0 {
                acc.insert(i);
            }
            if (gcd / i) % lcm == 0 {
                acc.insert(gcd / i);
            }

            acc
        })
        .len();

    println!("{count}");
}

fn get_lcm(a: i64, b: i64) -> i64 {
    a / get_gcd([a, b].into_iter()) * b
}

fn get_gcd<I>(nums: I) -> i64
where
    I: Iterator<Item = i64>,
{
    nums.reduce(|mut a, mut b| loop {
        if b == 0 {
            return a;
        }

        (a, b) = (b, a % b);
    })
    .unwrap()
}
