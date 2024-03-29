use std::cmp::Ordering;
use std::io;

#[derive(Eq, PartialEq, Debug)]
struct BigInt(Vec<u8>);

impl Ord for BigInt {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0
            .len()
            .cmp(&other.0.len())
            .then_with(|| self.0.iter().rev().cmp(other.0.iter().rev()))
    }
}

impl PartialOrd for BigInt {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines();
    let mut input = || input.next().unwrap();

    let n: i32 = input().parse().unwrap();

    for (a, b) in (0..n).map(|_| (get_paren_value(input()), get_paren_value(input()))) {
        // println!("{a:?} {b:?}");
        println!(
            "{}",
            match a.cmp(&b) {
                Ordering::Less => '<',
                Ordering::Equal => '=',
                Ordering::Greater => '>',
            }
        );
    }
}

fn get_paren_value(s: &str) -> BigInt {
    let mut paren_value = vec![0; s.len() / 2];
    let mut depth = 0;
    let mut prev_ch = '\0';

    for ch in s.chars() {
        match ch {
            '(' => depth += 1,
            ')' => {
                depth -= 1;

                if prev_ch == '(' {
                    paren_value[depth] += 1;
                }
            }
            _ => unreachable!(),
        }

        prev_ch = ch;
    }
    // println!("{paren_value:?}");
    while paren_value.len() > 0 && paren_value.last() == Some(&0) {
        paren_value.pop();
    }

    let mut carry = 0;
    let mut result: Vec<_> = paren_value
        .iter()
        .map(|&num| {
            let temp = carry + num;
            carry = temp >> 1;

            temp & 1
        })
        .collect();

    while carry > 0 {
        result.push(carry & 1);
        carry >>= 1;
    }

    BigInt(result)
}
