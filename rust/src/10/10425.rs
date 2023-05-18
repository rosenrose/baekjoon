use std::fmt::Write;
use std::io;

struct BigInt;

impl BigInt {
    fn rem(input: &str, m: i64) -> i64 {
        let mut pow_rem = 1;

        input.as_bytes().iter().rev().fold(0, |acc, ch| {
            let num = (ch - b'0') as i64;
            let rem = (num * pow_rem) % m;
            pow_rem = (pow_rem * 10) % m;

            (acc + rem) % m
        })
    }
}

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.lines();
    let mut output = String::new();

    const M: i64 = 1_000_000_009;
    let mut memo = vec![(0, 0); 100_000 + 1];
    memo[1] = (1, 1);

    for i in 2..memo.len() {
        memo[i] = (i, (memo[i - 2].1 + memo[i - 1].1) % M);
    }
    // println!("{memo:?}");
    memo.sort_unstable_by_key(|&(_, rem)| rem);

    for fibo_rem in input.skip(1).map(|s| BigInt::rem(s, M)) {
        let n = if fibo_rem == 1 {
            2
        } else {
            let i = memo
                .binary_search_by_key(&fibo_rem, |&(_, rem)| rem)
                .unwrap();

            memo[i].0
        };

        writeln!(output, "{n}").unwrap();
    }

    print!("{output}");
}
