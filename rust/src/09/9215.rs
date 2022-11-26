use std::fmt;
use std::ops::Add;

struct MixedFraction {
    integer: i32,
    numerator: i32,
    denominator: i32,
}

impl MixedFraction {
    fn from(integer: i32, numerator: i32, denominator: i32) -> Self {
        Self {
            integer,
            numerator,
            denominator,
        }
    }

    fn parse(input: &str) -> Self {
        match input.split([',', '/']).map(parse_int).collect::<Vec<_>>()[..] {
            [a, b, c] => Self::from(a, b, c),
            [b, c] => Self::from(0, b, c),
            [a] => Self::from(a, 0, 1),
            _ => Self::from(0, 0, 1),
        }
    }
}

impl Add for MixedFraction {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let lcm = get_lcm(self.denominator, other.denominator);

        let mut numerator =
            self.numerator * (lcm / self.denominator) + other.numerator * (lcm / other.denominator);
        let mut denominator = lcm;

        let gcd = get_gcd(numerator, denominator);
        (numerator, denominator) = (numerator / gcd, denominator / gcd);

        let integer = self.integer + other.integer + numerator / denominator;
        numerator %= denominator;

        Self {
            integer,
            numerator,
            denominator,
        }
    }
}

impl fmt::Display for MixedFraction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.integer == 0 && self.numerator == 0 {
            return write!(f, "0");
        }

        if self.integer != 0 {
            write!(f, "{}", self.integer).unwrap();
        }

        if self.integer != 0 && self.numerator != 0 {
            write!(f, ",").unwrap();
        }

        if self.numerator != 0 {
            write!(f, "{}/{}", self.numerator, self.denominator).unwrap();
        }

        write!(f, "")
    }
}

fn main() {
    let mut buf = String::new();

    for i in 1.. {
        read_line(&mut buf);
        let n = parse_int(buf.trim());

        if n == 0 {
            return;
        }

        let sum = (0..n)
            .map(|_| {
                read_line(&mut buf);
                MixedFraction::parse(buf.trim())
            })
            .reduce(|a, b| a + b)
            .unwrap();

        println!("Test {i}: {sum}");
    }
}

fn get_lcm(a: i32, b: i32) -> i32 {
    a / get_gcd(a, b) * b
}

fn get_gcd(mut a: i32, mut b: i32) -> i32 {
    loop {
        (a, b) = (b, a % b);

        if b == 0 {
            return a;
        }
    }
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}

fn parse_int(buf: &str) -> i32 {
    buf.parse().unwrap()
}
