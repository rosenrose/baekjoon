use std::fmt;
use std::ops::AddAssign;

struct Fraction(i64, i64);

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
        write!(f, "{}/{}", self.0, self.1)
    }
}

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let input = buf.trim();
    let Some((integer, decimal)) = input.split_once('.') else {
        println!("{}", Fraction(parse_int(input), 1));
        return;
    };

    let mut fraction = Fraction(parse_int(integer), 1);
    let mut tokens = decimal.split(['(', ')']);
    let non_repeat = tokens.next().unwrap();

    for (i, c) in non_repeat.char_indices() {
        fraction += Fraction(c as i64 - '0' as i64, 10_i64.pow(i as u32 + 1));
    }

    if let Some(repeating) = tokens.next() {
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
    }

    println!("{fraction}");
}

fn get_lcm(a: i64, b: i64) -> i64 {
    a / get_gcd(a, b) * b
}

fn get_gcd(mut a: i64, mut b: i64) -> i64 {
    loop {
        if b == 0 {
            return a;
        }

        (a, b) = (b, a % b);
    }
}

fn parse_int(buf: &str) -> i64 {
    buf.parse().unwrap()
}
