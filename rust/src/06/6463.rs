use std::io;

const DIGITS: usize = 18;
const POW: i128 = 10_i128.pow(DIGITS as u32);
const MAX: usize = 9999;

struct BigInt(Vec<i64>);

impl BigInt {
    fn mul(&self, other: i64) -> Self {
        let mut carry = 0;
        let mut result: Vec<_> = self
            .0
            .iter()
            .map(|&num| {
                let temp = carry + num as i128 * other as i128;
                carry = temp / POW;

                (temp % POW) as i64
            })
            .collect();

        if carry > 0 {
            result.push(carry as i64);
        }

        Self(result)
    }
}

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.lines().flat_map(str::parse::<usize>);

    let mut memo = [(); MAX + 1].map(|_| BigInt(vec![1]));

    for i in 1..=MAX {
        let mut factorial = memo[i - 1].mul(i as i64);

        while factorial.0[0] == 0 {
            factorial.0.remove(0);
        }

        let next = factorial.0.into_iter().take(2).collect();

        memo[i] = BigInt(next);
    }

    for n in input {
        let non_zero = memo[n].0[0]
            .to_string()
            .chars()
            .rev()
            .find(|&ch| ch != '0')
            .unwrap();

        println!("{n:>5} -> {non_zero}");
    }
}
