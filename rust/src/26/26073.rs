use std::fmt::Write;
use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());
    let mut output = String::new();

    for _ in 0..input.next().unwrap() {
        let (x, y) = (input.next().unwrap(), input.next().unwrap());
        let k = input.next().unwrap();

        let gcd = get_gcd((0..k).map(|_| input.next().unwrap()));

        if x % gcd != 0 || y % gcd != 0 {
            writeln!(output, "Gave up").unwrap();
            continue;
        }

        writeln!(output, "Ta-da").unwrap();
    }

    print!("{output}");
}

fn get_gcd<I>(nums: I) -> i32
where
    I: Iterator<Item = i32>,
{
    nums.reduce(|mut a, mut b| loop {
        if b == 0 {
            return a;
        }

        (a, b) = (b, a % b);
    })
    .unwrap()
}
