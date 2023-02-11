use std::fmt::Write;
use std::io;

struct BigInt;

impl BigInt {
    fn rem(input: &str, m: i64) -> i64 {
        let mut exp_rem = 1;

        input.chars().rev().fold(0, |acc, ch| {
            let num = ch as i64 - '0' as i64;
            let rem = (num * exp_rem) % m;
            exp_rem = (exp_rem * 10) % m;

            (acc + rem) % m
        })
    }
}

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.lines();
    let mut output = String::new();

    const M: i64 = 1_000_000_009;
    let mut cache = vec![(0, 0); 100_000 + 1];
    cache[1] = (1, 1);

    for i in 2..cache.len() {
        cache[i] = (i, (cache[i - 2].1 + cache[i - 1].1) % M);
    }
    // println!("{cache:?}");
    cache.sort_unstable_by_key(|&(_, rem)| rem);

    for fibo_rem in input.skip(1).map(|s| BigInt::rem(s, M)) {
        let n = if fibo_rem == 1 {
            2
        } else {
            let i = cache
                .binary_search_by_key(&fibo_rem, |&(_, rem)| rem)
                .unwrap();

            cache[i].0
        };

        writeln!(output, "{n}").unwrap();
    }

    print!("{output}");
}
