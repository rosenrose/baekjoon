use std::io;

struct BigInt {
    nums: Vec<u32>,
    radix: u32,
}

impl BigInt {
    fn parse(input: &str, radix: u32) -> Self {
        Self {
            nums: input.chars().rev().map(|c| c as u32 - '0' as u32).collect(),
            radix,
        }
    }

    fn rem(&self, m: u32) -> u32 {
        let mut exp_rem = 1;

        self.nums.iter().fold(0, |acc, num| {
            let rem = (num * exp_rem) % m;
            exp_rem = (exp_rem * self.radix) % m;

            (acc + rem) % m
        })
    }
}

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();

    while let (Some(b), Some(p), Some(m)) = (input.next(), input.next(), input.next()) {
        if b == "0" {
            return;
        }

        let radix: u32 = b.parse().unwrap();
        let p = BigInt::parse(p, radix);
        let m = u32::from_str_radix(m, radix).unwrap();

        let mut rem = p.rem(m);
        let mut result = Vec::new();
        // println!("{rem}");
        loop {
            result.push(rem % radix);
            rem /= radix;

            if rem == 0 {
                break;
            }
        }

        for num in result.iter().rev() {
            print!("{num}");
        }
        println!("");
    }
}
