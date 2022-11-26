use std::fmt;
use std::ops::Add;

struct Fraction {
    numerator: i32,
    denominator: i32,
}

impl Add for Fraction {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let lcm = get_lcm(self.denominator, other.denominator);

        let numerator =
            self.numerator * (lcm / self.denominator) + other.numerator * (lcm / other.denominator);
        let denominator = lcm;

        let gcd = get_gcd(numerator, denominator);

        Self {
            numerator: numerator / gcd,
            denominator: denominator / gcd,
        }
    }
}

impl fmt::Display for Fraction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.numerator, self.denominator)
    }
}

fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    let parse_int = |s: &str| s.parse::<i32>().unwrap();
    let mut nums = buf.split_whitespace().map(parse_int);

    let fraction1 = Fraction {
        numerator: nums.next().unwrap(),
        denominator: nums.next().unwrap(),
    };

    read_line(&mut buf);
    nums = buf.split_whitespace().map(parse_int);

    let fraction2 = Fraction {
        numerator: nums.next().unwrap(),
        denominator: nums.next().unwrap(),
    };

    println!("{}", fraction1 + fraction2);
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
