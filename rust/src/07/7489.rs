use std::io;

const DIGITS: usize = 18;
const POW: i128 = 10_i128.pow(DIGITS as u32);

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
    let input = buf.lines().flat_map(str::parse::<i64>);

    for n in input.skip(1) {
        let factorial = (1..=n).fold(BigInt(vec![1]), |mut acc, num| {
            acc = acc.mul(num);

            while acc.0[0] == 0 {
                acc.0.remove(0);
            }

            acc
        });

        let non_zero = factorial.0[0]
            .to_string()
            .chars()
            .rev()
            .find(|&c| c != '0')
            .unwrap();

        println!("{non_zero}");
    }
}
