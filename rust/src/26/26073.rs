use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut input = || input.next().unwrap();
    let mut output = String::new();

    for _ in 0..input() {
        let (x, y) = (input(), input());
        let gcd = get_gcd((0..input()).map(|_| input()));

        if x % gcd != 0 || y % gcd != 0 {
            writeln!(output, "Gave up").unwrap();
            continue;
        }

        writeln!(output, "Ta-da").unwrap();
    }

    print!("{output}");
}

fn get_gcd(nums: impl Iterator<Item = i32>) -> i32 {
    nums.reduce(|mut a, mut b| loop {
        if b == 0 {
            return a;
        }

        (a, b) = (b, a % b);
    })
    .unwrap()
}
