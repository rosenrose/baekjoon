use std::fmt;
use std::io;
use std::ops::AddAssign;

struct Fraction(i32, i32);

impl AddAssign for Fraction {
    fn add_assign(&mut self, other: Self) {
        let lcm = get_lcm(self.1, other.1);

        let numerator = self.0 * (lcm / self.1) + other.0 * (lcm / other.1);
        let denominator = lcm;

        let gcd = get_gcd(numerator, denominator);

        *self = Self(numerator / gcd, denominator / gcd);
    }
}

impl fmt::Display for Fraction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} / {}", self.0, self.1)
    }
}

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();

    for input in buf.lines() {
        let (integer, decimal) = input.split_once('.').unwrap();
        let mut fraction = Fraction(integer.parse().unwrap(), 1);

        let mut it = decimal.split(['(', ')']);
        let non_repeat = it.next().unwrap();
        let repeating = it.next().unwrap();

        for (i, ch) in non_repeat.as_bytes().iter().enumerate() {
            fraction += Fraction((ch - b'0') as i32, 10_i32.pow(i as u32 + 1));
        }

        fraction += Fraction(
            repeating.parse().unwrap(),
            format!(
                "{}{}",
                "9".repeat(repeating.len()),
                "0".repeat(non_repeat.len())
            )
            .parse()
            .unwrap(),
        );

        println!("{input} = {fraction}");
    }
}

fn get_lcm(a: i32, b: i32) -> i32 {
    a / get_gcd(a, b) * b
}

fn get_gcd(mut a: i32, mut b: i32) -> i32 {
    loop {
        if b == 0 {
            return a;
        }

        (a, b) = (b, a % b);
    }
}
