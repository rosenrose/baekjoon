use std::io;

struct BigInt;

impl BigInt {
    fn rem(input: &str, m: u32, radix: u32) -> u32 {
        let mut exp_rem = 1;

        input.chars().rev().fold(0, |acc, ch| {
            let num = ch as u32 - '0' as u32;
            let rem = (num * exp_rem) % m;
            exp_rem = (exp_rem * radix) % m;

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
        let m = u32::from_str_radix(m, radix).unwrap();
        let mut rem = BigInt::rem(p, m, radix);
        // println!("{rem}");
        let mut result = Vec::new();

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