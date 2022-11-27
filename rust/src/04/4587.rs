use std::ops::Sub;

#[derive(Copy, Clone)]
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
    }

    fn parse(input: &str) -> Self {
        let mut nums = input.split_whitespace().map(|s| s.parse::<i64>().unwrap());
        Self::from(nums.next().unwrap(), nums.next().unwrap())
    }

    fn unit(int: i64) -> Self {
        Self::from(1, int)
    }

    fn reduced(self) -> Self {
        let gcd = get_gcd(self.numerator, self.denominator);
        let gcd = if gcd < 0 { -gcd } else { gcd };

        Self {
            numerator: self.numerator / gcd,
            denominator: self.denominator / gcd,
        }
    }
}

impl Sub for Fraction {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self::from(
            self.numerator * other.denominator - other.numerator * self.denominator,
            self.denominator * other.denominator,
        )
        .reduced()
    }
}

fn main() {
    let mut buf = String::new();

    loop {
        read_line(&mut buf);

        if buf.trim() == "0 0" {
            return;
        }

        let mut fraction = Fraction::parse(buf.trim());

        while fraction.numerator > 1 {
            for i in (fraction.denominator / fraction.numerator) + 1.. {
                let next = fraction - Fraction::unit(i);

                if next.numerator > 0 && next.denominator < 1_000_000 {
                    fraction = next;
                    print!("{i} ");
                    break;
                }
            }
        }

        println!("{}", fraction.denominator);
    }
}

fn get_gcd(mut a: i64, mut b: i64) -> i64 {
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
