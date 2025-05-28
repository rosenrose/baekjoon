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

const MAX: usize = 3_000_000 / 2;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines();

    let n: i32 = input.next().unwrap().parse().unwrap();

    for [a, b] in (0..n).map(|_| [(); 2].map(|_| get_paren_value(input.next().unwrap()))) {
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
    let mut paren_value = [0; MAX];
    let mut paren_value_len = paren_value.len();
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
    while paren_value_len > 0 && paren_value[paren_value_len - 1] == 0 {
        paren_value_len -= 1;
    }

    let mut carry = 0;
    let mut result: Vec<_> = paren_value[..paren_value_len]
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
