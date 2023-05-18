use std::ops::Add;

struct Fraction(i64, i64);

impl Fraction {
    fn reduced(numerator: i64, denominator: i64) -> Self {
        let gcd = get_gcd(numerator, denominator).abs();

        Self(numerator / gcd, denominator / gcd)
    }
}

impl Add for Fraction {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::reduced(self.0 * other.1 + other.0 * self.1, self.1 * other.1)
    }
}

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let (integer, decimal) = buf.trim().split_once('.').unwrap();
    let integer = integer.parse().unwrap();
    let fraction = decimal
        .as_bytes()
        .iter()
        .enumerate()
        .map(|(i, ch)| Fraction((ch - b'0') as i64, 10_i64.pow(i as u32 + 1)))
        .fold(Fraction(integer, 1), |acc, a| acc + a);

    let papers = fraction.1;
    let mut sum = fraction.0;
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
