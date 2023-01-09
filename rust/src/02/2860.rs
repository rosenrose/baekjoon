use std::ops::Add;

struct Fraction {
    numerator: i64,
    denominator: i64,
}

impl Fraction {
    fn from(numerator: i64, denominator: i64) -> Self {
        Self {
            numerator,
            denominator,
        }
        .reduced()
    }

    fn reduced(self) -> Self {
        let gcd = get_gcd(self.numerator, self.denominator).abs();

        Self {
            numerator: self.numerator / gcd,
            denominator: self.denominator / gcd,
        }
    }
}

impl Add for Fraction {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::from(
            self.numerator * other.denominator + other.numerator * self.denominator,
            self.denominator * other.denominator,
        )
    }
}

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let mut p = buf.trim().split('.');
    let (integer, decimal) = (p.next().unwrap(), p.next().unwrap());

    let integer = parse_int(integer);
    let fraction = decimal
        .char_indices()
        .map(|(i, c)| Fraction::from(c as i64 - '0' as i64, 10_i64.pow(i as u32 + 1)))
        .fold(Fraction::from(integer, 1), |acc, a| acc + a);

    let papers = fraction.denominator;
    let mut sum = fraction.numerator;
    let mut num_counts = [0; 5];

    sum -= papers;

    for i in (1..=4).rev() {
        num_counts[i as usize] += sum / i;
        sum %= i;
    }

    num_counts[0] = papers - num_counts.iter().sum::<i64>();

    for count in num_counts {
        print!("{count} ");
    }
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
    buf.trim().parse().unwrap()
}
